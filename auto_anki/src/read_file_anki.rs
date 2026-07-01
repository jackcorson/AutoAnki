use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use reqwest::Client;
use serde_json::json;

pub async fn handle_reading_from_file(file_path: String) -> Result<(), Box<dyn Error>> {
    let words_and_defs = read_word_file(file_path)?;
    for (word, def) in words_and_defs {
        let in_deck = check_exists_json(&word).await?;
        if !in_deck {
            println!("Adding '{word}' as a new flashcard!");
            format_as_json(word.as_str().trim().to_string(), def.as_str().trim().to_string()).await?;
        }
        else {
            println!("{word} already exists as a flashcard");
        }
    }
    Ok(())
}

fn read_word_file(file_path: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut wordsndefs = HashMap::new();

    let mut file = File::open(file_path.as_str())?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Could not read contents");

    let list = contents.lines();

    for word in list { 
        let mut parts = word.split("-");

        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            wordsndefs.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    return Ok(wordsndefs)
}

pub async fn check_exists_json(word: &str) -> Result<bool, Box<dyn Error>> {
    let request = json!({
        "action": "findNotes",
        "version": 6,
        "params": {
            "query": format!("deck:words_and_phrases Front:\"{}\"", word.trim()),
        }
    });
    
    let value = get_from_anki(request).await?;

    let is_empty = value["result"]
    .as_array()
    .unwrap()
    .is_empty();

    if is_empty {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn format_as_json(word: String, def: String) -> Result<(), Box<dyn Error>> {
    let json = json!({
        "action": "addNote",
        "version": 6,
        "params": {
            "note": {
                "deckName": "words_and_phrases",
                "modelName": "Basic",
                "fields": {
                    "Front": word.as_str(),
                    "Back": def.as_str(),
                },
                "tags": ["rust"]
            }
        }
    });

    send_to_anki(json).await?;
    
    Ok(())
}

async fn get_from_anki(request: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> { 
    let client = Client::new();

    let response = client
        .post("http://localhost:8765")
        .json(&request)
        .send()
        .await?;

    let value: serde_json::Value = response.json().await?;

    Ok(value)
}

async fn send_to_anki(request: serde_json::Value) -> Result<(), Box<dyn Error>> { 
    let client = Client::new();

    client
        .post("http://localhost:8765")
        .json(&request)
        .send()
        .await?;

    Ok(())
}
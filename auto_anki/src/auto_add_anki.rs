use std::error::Error;
use crate::{read_file_anki::format_as_json, read_file_anki::check_exists_json};
use std::env;
use reqwest::Client;
use serde_json::Value;

pub async fn handle_auto_add(words: Vec<String>) -> Result<(), Box<dyn Error>>{
    for word in words {
        let in_deck = check_exists_json(&word).await?;
        if !in_deck {
            println!("Adding '{word}' as a new flashcard!");
            let definition = get_definition(&word).await?;
            format_as_json(word.as_str().trim().to_string(), definition.as_str().trim().to_string()).await?;
        }
        else {
            println!("{word} already exists as a flashcard");
        }
    }
    Ok(())
}

async fn get_definition(word: &str) -> Result<String, Box<dyn Error>>{
    let key = match env::var("MERRIAM_API_KEY") {
        Ok( val ) => val,
        Err( e ) => panic!("Could not retrieve API key {e}"),
    };

    let word_to_get = word.trim();

    let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{word_to_get}?key={key}");

    let value: Value = Client::new()
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    let defs = value
        .get(0)
        .and_then(|v| v.get("shortdef"))
        .and_then(|v| v.as_array())
        .ok_or("No shortdef found")?;

    let definition = defs
        .iter()
        .take(2)
        .filter_map(|d| d.as_str())
        .collect::<Vec<_>>()
        .join("; ");

    Ok(definition)
}
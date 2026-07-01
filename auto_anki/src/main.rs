/*
    Overall Idea:

    Want a program that creates flashcards using Anki api.
    First I need it to read from a file with all of the words and their definitions.
    Then after that I want it to be able to pull from the interenet if I just input one word.
    So maybe need a CLI that either reads from a file or just adds a list of words to the flashcards.
    
    APIs needed:
        * Anki
        * Merriam-Webster ?? (If possible. If not, some other dictionary database will do).
*/

mod read_file_anki;
mod auto_add_anki;
use clap::Parser;

use crate::{auto_add_anki::handle_auto_add, read_file_anki::handle_reading_from_file};
use std::error::Error;

#[derive(Parser)]
enum Decision {

    #[command(name = "readfile")]
    ReadFile {
        file_path: String,
    },

    #[command(name = "autoadd")]
    AutoAdd {
        words: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Decision::parse();

    match args {
        Decision::ReadFile { file_path } => handle_reading_from_file(file_path).await?,
        Decision::AutoAdd { words } => handle_auto_add(words).await?,
    }

    Ok(())
}

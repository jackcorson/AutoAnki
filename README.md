# Anki Auto Add

A command-line tool written in Rust that automates adding vocabulary flashcards to Anki.

Instead of manually creating cards one by one, this tool can either:

* Import words and definitions from a text file.
* Automatically fetch definitions from the Merriam-Webster Dictionary API and create flashcards for you.

Before adding a card, the program checks your Anki deck through AnkiConnect to avoid creating duplicate flashcards.

---

## Features

* Import a list of words and definitions from a file.
* Automatically retrieve definitions using the Merriam-Webster Dictionary API.
* Prevent duplicate flashcards by checking your Anki deck first.
* Fast asynchronous HTTP requests using Tokio and Reqwest.
* Built entirely in Rust.

---

## Requirements

* Rust
* Anki (With a deck named 'words_and_phrases' created)
* AnkiConnect add-on running in Anki
* A Merriam-Webster Dictionary API key (only required for automatic definition lookup)

---

## Installation

Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

Build the project:

```bash
cargo build --release
```

---

## Setup

### 1. Install AnkiConnect

Install the AnkiConnect add-on and make sure Anki is running before executing the program.

The application communicates with Anki through:

```
http://localhost:8765
```

### 2. Set your Merriam-Webster API key

Create an environment variable named:

```text
MERRIAM_API_KEY
```

For example:

**macOS/Linux**

```bash
export MERRIAM_API_KEY=your_api_key
```

**Windows (PowerShell)**

```powershell
$env:MERRIAM_API_KEY="your_api_key"
```

---

## Usage

### Option 1: Import from a file

Create a text file where each line is formatted as:

```text
word - definition
```

Example:

```text
ephemeral - lasting for a very short time
ubiquitous - existing or appearing everywhere
brass tacks - practical details
```

Run:

```bash
cargo run -- readfile words.txt
```

Each entry is checked against your Anki deck before being added.

---

### Option 2: Automatically fetch definitions

Provide one or more words on the command line:

```bash
cargo run -- autoadd ephemeral ubiquitous paradigm
```

The program will:

1. Check if each word already exists in your Anki deck.
2. Retrieve its definition from the Merriam-Webster Dictionary API.
3. Add the flashcard to Anki if it does not already exist.

---

## Anki Deck

The program currently adds cards to:

```
words_and_phrases
```

using Anki's built-in **Basic** note type.

Flashcards are created as:

| Front       | Back       |
| ----------- | ---------- |
| Word/Phrase | Definition |

---

## Project Structure

```
src/
├── main.rs              # CLI entry point
├── auto_add_anki.rs     # Merriam-Webster API integration
└── read_file_anki.rs    # File import and AnkiConnect logic
```

---

## Technologies Used

* Rust
* Tokio
* Reqwest
* Clap
* Serde JSON
* AnkiConnect
* Merriam-Webster Dictionary API


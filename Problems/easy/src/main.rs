// Challenge: Word Frequency Counter (CLI)
// Write a small command-line program that:
// 1. Reads a text file whose path is provided as a command-line argument
// 2. Counts how many times each word appears (case-insensitive)
// 3. Prints the top 10 most frequent words with their counts.
// 4. Ignores punctuation

// Requirements
// - Use std::env for reading CLI arguments.
// - Use std::fs to read the file.
// - Use a 'HashMap<String, usize>' for counts.
// - Keep the implementation simple and idiomatic.
// - Comment your code clearly and concisely.

// Example CLI usage
// 'cargo run -- text.txt'

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// query is optional for counting a specific word
struct Config {
    query: Option<String>,
    path: String,
}

impl Config {
    // constructor
    fn new(args: &[String]) -> Result<Config, String> {
        // checking if at least one argument exists
        let path = match args.get(1) {
            Some(p) => p.to_string(),
            None => return Err("Not enough arguments provided!".to_string()),
        };
        let query = args.get(2).cloned();

        Ok(Config { query, path })
    }
}

// searches for words and counts occurrences
fn search(query: Option<&str>, contents: &str) -> Result<HashMap<String, usize>, String> {
    // Initializing empty HashMap
    let mut map: HashMap<String, usize> = HashMap::new();

    match query {
        // If query exists only search and count it
        Some(q) => {
            if contents.is_empty() {
                return Err("File is empty!".to_string());
            };
            let count = contents.matches(q).count();

            map.insert(q.to_string(), count);

            Ok(map)
        }
        // If query doesn't exist count every occurrence of each word
        _ => {
            if contents.is_empty() {
                return Err("File is empty!".to_string());
            };

            for word in contents.split_whitespace() {
                *map.entry(word.to_owned()).or_default() += 1;
            }

            Ok(map)
        }
    }
}

// runs the application
fn execute(config: Config) -> Result<(), Box<dyn Error>> {
    // Reads file contents as a string
    let file: String = fs::read_to_string(config.path)?
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect();

    // applies the search function
    let results = search(config.query.as_deref(), &file);

    // Prints the word and count
    for (word, count) in results? {
        println!("{word}: {count}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = execute(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

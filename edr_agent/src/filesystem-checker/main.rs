use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, BufReader};
use std::path::Path;
use serde::{Serialize, Deserialize};
use bincode;
use clap::{arg, Command};


const HASHES_FILE: &str = "hashes.txt";         // Original file with SHA-256 hashes
const STORED_HASHES_FILE: &str = "stored_hashes.bin"; // Serialized HashMap storage

#[derive(Serialize, Deserialize)]
struct StoredHashes {
    hashes: HashMap<String, String>,
}



fn cli() -> Command {
    Command::new("filesystem-checker")
        .about("Check for added/modified files across the filesystem")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("load")
                .about("load baseline hashes")
                .arg(arg!(-f --file <HASHES_FILE> "File with sha256 hashes"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("test")
                .about("test against baseline hashes")
                .arg(arg!(-f --file <HASHES_FILE> "File with sha256 hashes"))
                .arg_required_else_help(true)
        )
}


fn handle_args(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("greet", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let times = sub_matches.get_one::<String>("times")
                .expect("defaulted")
                .parse::<u32>()
                .expect("valid number");
            for _ in 0..times {
                println!("Hello, {}!", name);
            }
        }
        Some(("calc", sub_matches)) => {
            let x = sub_matches.get_one::<String>("X")
                .expect("required")
                .parse::<i32>()
                .expect("valid number");
            let y = sub_matches.get_one::<String>("Y")
                .expect("required")
                .parse::<i32>()
                .expect("valid number");
            println!("{} + {} = {}", x, y, x + y);
        }
        _ => unreachable!(),
    }
}




// Load hashes from `hashes.txt` into a HashMap
fn load_hashes_from_txt(file_path: &str) -> HashMap<String, String> {
    let mut file_hash_map = HashMap::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.split_whitespace().collect();
                if parts.len() == 2 {
                    file_hash_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    } else {
        eprintln!("Error: Could not open {}", file_path);
    }

    file_hash_map
}

// Save HashMap to a binary file for persistence
fn save_hashes_to_file(hashes: &HashMap<String, String>, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    let stored_hashes = StoredHashes { hashes: hashes.clone() };

    bincode::serialize_into(writer, &stored_hashes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e)))
}

// Load persisted HashMap from binary file
fn load_hashes_from_file(file_path: &str) -> io::Result<HashMap<String, String>> {
    if !Path::new(file_path).exists() {
        return Ok(HashMap::new()); // Return empty HashMap if file doesn't exist
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let stored_hashes: StoredHashes = bincode::deserialize_from(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Deserialization error: {}", e)))?;
    
    Ok(stored_hashes.hashes)
}

fn main() {
// let matches = cli().get_matches();
//     handle_args(matches);


    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <load|test>", args[0]);
        return;
    }

    let command = args[1].as_str();

    match command {
        "load" => {
            let file_hash_map = load_hashes_from_txt(HASHES_FILE);
            if save_hashes_to_file(&file_hash_map, STORED_HASHES_FILE).is_ok() {
                println!("Saved {} hashes to {}", file_hash_map.len(), STORED_HASHES_FILE);
            } else {
                eprintln!("Error saving hashes.");
            }
        }
        "test" => {
            match load_hashes_from_file(STORED_HASHES_FILE) {
                Ok(file_hash_map) => {
                    println!("Loaded {} hashes from {}", file_hash_map.len(), STORED_HASHES_FILE);
                    
                    let new_hashes = load_hashes_from_txt(HASHES_FILE);
                    
                    for (hash, filename) in &new_hashes {
                        if !file_hash_map.contains_key(hash) {
                            println!("Modified or new file: {}", filename);
                        }
                    }
                }
                Err(e) => eprintln!("Error loading stored hashes: {}", e),
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'load' or 'test'.");
        }
    }
}

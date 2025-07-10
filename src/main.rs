use arboard::Clipboard;
use clap::{Parser, Subcommand};
use colored::*;
use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

fn history_path() -> PathBuf {
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("tikcopy_history.json");
    path
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClipItem {
    text: String,
}

#[derive(Parser)]
#[command(name = "TikCopy")]
#[command(about = "Clipboard history manager for Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add,
    Use {
        index: usize,
    },
    Delete {
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let path = history_path();
    let mut history: Vec<ClipItem> = fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    match &cli.command {
        Commands::List => {
            if history.is_empty() {
                println!("{}", "No clipboard history found.".yellow());
            } else {
                for (i, item) in history.iter().enumerate() {
                    let index = format!("{}:", i + 1).blue();
                    println!("{} {}", index, item.text.green());
                }
            }
        }
        Commands::Add => {
            let mut clipboard = Clipboard::new().unwrap();
            let text = if atty::isnt(atty::Stream::Stdin) {
                use std::io::Read;
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer).unwrap();
                buffer.trim().to_string()
            } else {
                clipboard.get_text().unwrap_or_default()
            };

            if text.is_empty() {
                println!("{}", "Clipboard is empty.".red());
                return;
            }

            history.insert(0, ClipItem { text });
            if history.len() > 50 {
                history.truncate(50);
            }
            fs::write(&path, serde_json::to_string_pretty(&history).unwrap()).unwrap();
            println!("{}", "✔ Added to history.".green());
        }
        Commands::Use { index } => {
            if let Some(item) = history.get(index - 1) {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(&item.text).unwrap();
                println!("{} {}", "✔ Set clipboard to:".green(), item.text.cyan());
            } else {
                println!("{}", "Invalid index.".red());
            }
        }
        Commands::Delete { index } => {
            if *index == 0 || *index > history.len() {
                println!("{}", "Invalid index.".red());
            } else {
                history.remove(index - 1);
                fs::write(&path, serde_json::to_string_pretty(&history).unwrap()).unwrap();
                println!("{}", "🗑 Deleted.".green());
            }
        }
    }
}

use colored::*;
use std::env;
use std::io::{self, Write};

use crate::help::help_message;

mod actions;
mod help;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback_message = String::from("Check usage with gii --help");

    if args.len() == 1 {
        println!("{}", fallback_message.red());
        std::process::exit(1);
    }

    if args[1] == "cp" {
        if args.len() < 3 {
            println!("{}", fallback_message.red());
            std::process::exit(1);
        }

        let commit_message = &args[2];

        staging();

        fn staging() {
            println!("");
            println!("Staging options:");
            println!("1. All files");
            println!("2. Specify which files");

            let mut choice = String::new();
            print!("Enter your choice (1 or 2): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => actions::add_all_files(),
                "2" => actions::add_specific_file(),
                _ => {
                    eprintln!("{}", "Invalid choice".red());
                    staging();
                }
            }
        }

        actions::commit_changes(commit_message);
        actions::push_to_origin();
    } else if args[1] == "ar" {
        if let Some(args) = std::env::args().nth(2) {
            if let Ok(_) = args.parse::<i32>() {
            } else {
                let url = &args;
                actions::add_remote_origin(url);
            }
        } else {
            println!("{}", fallback_message.red());
            std::process::exit(1);
        }
    } else if args[1] == "--help" {
        help_message()
    } else if args[1] == "--version" {
        println!("gitdonewithit (gii)");
        println!("Made by th0jensen");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    } else {
        println!("{}", fallback_message.red());
        std::process::exit(1);
    }
}

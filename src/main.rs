use crate::helper::*;
use colored::*;
use std::env;

mod helper;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback_message = String::from("Check usage with gii help");

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
        actions::staging();
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
    } else if args[1] == "mr" {
        if let Some(args) = std::env::args().nth(2) {
            if let Ok(_) = args.parse::<i32>() {
            } else {
                let url = &args;
                actions::modify_remote_origin(url);
            }
        } else {
            println!("{}", fallback_message.red());
            std::process::exit(1);
        }
    } else if args[1] == "status" {
        actions::status();
    } else if args[1] == "log" {
        actions::log()
    } else if args[1] == "help" {
        help::help_message()
    } else if args[1] == "--version" {
        println!("gitdonewithit (gii)");
        println!("Made by th0jensen");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    } else {
        println!("{}", fallback_message.red());
        std::process::exit(1);
    }
}

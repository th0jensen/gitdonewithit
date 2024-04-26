use crate::helper::*;
use colored::*;
use std::env;

mod helper;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback_message = String::from("Check usage with gii --h");

    if args.len() == 1 {
        println!("{}", fallback_message.red());
        std::process::exit(1);
    }

    if args[1] == "cp" || args[1] == "--commit-push" {
        if args.len() < 3 {
            println!("{}", fallback_message.red());
            std::process::exit(1);
        }
        let commit_message = &args[2];
        actions::staging();
        actions::commit_changes(commit_message);
        actions::push_to_origin();
    } else if args[1] == "ar" || args[1] == "--add-remote" {
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
    } else if args[1] == "mr" || args[1] == "--modify-remote" {
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
    } else if args[1] == "status" || args[1] == "--s" {
        actions::status();
    } else if args[1] == "log" || args[1] == "--l" {
        actions::log()
    } else if args[1] == "help" || args[1] == "--h" {
        help::help_message(&args[2..])
    } else if args[1] == "version" || args[1] == "--v" {
        version::version_message()
    } else {
        println!("{}", fallback_message.red());
        std::process::exit(1);
    }
}

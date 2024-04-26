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
        actions::commit_push(&args[2..])
    } else if args[1] == "ar" || args[1] == "--add-remote" {
        actions::add_remote_origin(&args[2..]);
    } else if args[1] == "mr" || args[1] == "--modify-remote" {
        actions::modify_remote_origin(&args[2..]);
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

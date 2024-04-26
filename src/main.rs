use crate::helper::*;
use colored::*;
use std::{env, process::ExitCode};

mod helper;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("{}", "ERROR: Invalid command".red());
        println!("Check usage with gii --h");
        return ExitCode::FAILURE;
    } else {
        match &args[1][..] {
            "cp" | "--commit-push" => actions::commit_push(&args[2..]),
            "ar" | "--add-remote" => actions::add_remote_origin(&args[2..]),
            "mr" | "--modify-remote" => actions::modify_remote_origin(&args[2..]),
            "push" | "--p" => actions::push(),
            "pull" | "--pl" => actions::pull(),
            "fetch" | "--f" => actions::fetch(),
            "status" | "--s" => actions::status(),
            "log" | "--l" => actions::log(),
            "help" | "--h" => help::help_message(&args[2..]),
            "version" | "--v" => version::version_message(),
            _ => {
                eprintln!("{}", "ERROR: Check usage with gii --h".red());
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}

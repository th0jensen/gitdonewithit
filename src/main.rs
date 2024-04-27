use crate::helper::actions::*;
use crate::helper::help::*;
use crate::helper::version::*;
use ansi_term::Colour;
use crossterm::{cursor, execute, terminal};
use std::env;
use std::io::Result;

mod helper;

fn main() -> Result<()> {
    execute!(
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help_message(&args[1..]);
    } else {
        match &args[1][..] {
            "cp" | "--commit-push" => commit_push(&args[2..]),
            "ar" | "--add-remote" => add_remote_origin(&args[2..]),
            "mr" | "--modify-remote" => modify_remote_origin(&args[2..]),
            "push" | "--p" => push(),
            "pull" | "--pl" => pull(),
            "fetch" | "--f" => fetch(),
            "status" | "--s" => status(),
            "log" | "--l" => log(),
            "help" | "--h" => help_message(&args[2..]),
            "version" | "--v" => version_message(),
            _ => {
                eprintln!("{}", Colour::Red.paint("ERROR: Invalid command"));
                println!("Check usage with gii help");
            }
        }
    }

    Ok(())
}

use std::env;
use std::io::{self, Write};

mod actions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "cp" {
        if args.len() < 3 {
            eprintln!("Usage: {} cp 'commit_message'", args[0]);
            std::process::exit(1);
        }

        let commit_message = &args[2];

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
                eprintln!("Invalid choice");
                std::process::exit(1);
            }
        }

        actions::commit_changes(commit_message);
        actions::push_to_origin();
    } else if args[1] == "ar" {
        if args.len() < 3 {
            eprintln!("Usage: {} ar repository [url]", args[0]);
            std::process::exit(1);
        }

        let repo_name = &args[2];

        if let Err(e) = env::set_current_dir(repo_name) {
            eprintln!("Error changing directory: {}", e);
            std::process::exit(1);
        }

        if args.len() == 3 {
            let url = &args[3];
            actions::add_remote_origin(url);
        }
    } else {
        eprintln!("Usage: gitdonewithit cp 'commit_message'");
        eprintln!("-------------------- ar repository url");
        std::process::exit(1);
    }
}

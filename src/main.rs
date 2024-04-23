use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: gitdonewithit cp 'commit_message'");
        eprintln!("-------------------- ar repository url");
        std::process::exit(1);
    }

    if args[1] == "cp" {
        if args.len() < 3 {
            eprintln!("Usage: {} cp 'commit_message'", args[0]);
            std::process::exit(1);
        }

        let commit_message = &args[2];

        println!("Staging options:");
        println!("1. All files (.)");
        println!("2. Specific file");

        let mut choice = String::new();
        print!("Enter your choice (1 or 2): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_all_files(),
            "2" => add_specific_file(),
            _ => {
                eprintln!("Invalid choice");
                std::process::exit(1);
            }
        }

        commit_changes(commit_message);
        push_to_origin();
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
            add_remote_origin(url);
        }
    } else {
        eprintln!("Usage: gitdonewithit cp 'commit_message'");
        eprintln!("-------------------- ar repository url");
        std::process::exit(1);
    }
}

fn add_all_files() {
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add .");
    if !output.status.success() {
        eprintln!(
            "Error adding files: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        eprintln!("Succuessfully added all files");
    }
}

fn add_specific_file() {
    print!("Enter the file name(s): ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap();
    let output = Command::new("git")
        .arg("add")
        .arg(file_name.trim())
        .output()
        .expect("Failed to execute git add");
    if !output.status.success() {
        eprintln!(
            "Error adding file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        eprintln!("Successfully added files: {}", &file_name);
    }
}

fn commit_changes(commit_message: &str) {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to execute git commit");
    if !output.status.success() {
        eprintln!(
            "Error committing changes: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        eprintln!("Successfully committed with message: '{}'", commit_message)
    }
}

fn push_to_origin() {
    print!("Do you want to push to origin main? (y/n): ");
    io::stdout().flush().unwrap();
    let mut push_choice = String::new();
    io::stdin().read_line(&mut push_choice).unwrap();

    if push_choice.trim().to_lowercase() == "y" {
        let output = Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("main")
            .output()
            .expect("Failed to execute git push");
        if !output.status.success() {
            eprintln!(
                "Error pushing changes: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        } else {
            eprintln!("Successfully pushed to remote! 🎉");
        }
    } else if push_choice.trim().to_lowercase() == "n" {
        eprintln!("Not pushing to remote");
    } else {
        eprintln!("Invalid input");
        push_to_origin();
    }
}

fn add_remote_origin(url: &str) {
    let output = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .output()
        .expect("Failed to execute git remote add origin");
    if !output.status.success() {
        eprintln!(
            "Error adding remote origin: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        eprintln!("Remote origin added successfully.");
    }
}

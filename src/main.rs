use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <repo_name> <commit_message>", args[0]);
        std::process::exit(1);
    }

    let repo_name = &args[1];
    let commit_message = &args[2];

    if let Err(e) = env::set_current_dir(repo_name) {
        eprintln!("Error changing directory: {}", e);
        std::process::exit(1);
    }

    println!("Select an option:");
    println!("1. All files (.)");
    println!("2. Specific file");

    let mut choice = String::new();
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
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
            }
        }
        "2" => {
            print!("Enter the file name: ");
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
            }
        }
        _ => {
            eprintln!("Invalid choice");
            std::process::exit(1);
        }
    }

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
    }

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
        }
    }
}

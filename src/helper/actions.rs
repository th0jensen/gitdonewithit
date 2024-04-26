use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub fn commit_push(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", "ERROR: Invalid command".red());
        println!("Check usage with gii --h cp");
    } else {
        let commit_message = &args[0];
        staging();
        commit_changes(commit_message);
        push_to_origin();
    }
}

fn staging() {
    println!("");
    println!("Staging options:");
    println!("┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄");
    println!("1. All files");
    println!("2. Specify which files");
    println!("3. Interactive mode");

    let mut choice = String::new();
    print!("Enter your choice ( 1 / 2 / 3 ): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => add_all_files(),
        "2" => add_specific_files(),
        "3" => interactive_mode(),
        _ => {
            eprintln!("{}", "Invalid choice".red());
            println!("Trying again...");
            staging();
        }
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
            "ERROR: Failed adding files: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", "Succuessfully added all files".green());
    }
}

fn add_specific_files() {
    print!("Enter the file path(s) (separated by spaces): ");
    io::stdout().flush().unwrap();
    let mut file_names = String::new();
    io::stdin().read_line(&mut file_names).unwrap();
    let file_name_list: Vec<&str> = file_names.trim().split_whitespace().collect();

    for file_name in file_name_list {
        let output = Command::new("git")
            .arg("add")
            .arg(file_name)
            .output()
            .expect("Failed to execute git add");
        if !output.status.success() {
            eprintln!(
                "ERROR: Failed adding file(s): {}",
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            println!(
                "{}",
                format!("Sucessfully added file: {}", file_name).green()
            );
        }
    }
}

fn interactive_mode() {
    let status = Command::new("git")
        .arg("add")
        .arg("-i")
        .status()
        .expect("Failed to execute git add -i");
    if !status.success() {
        eprintln!("ERROR: Failed adding files: {}", status.to_string().red());
        std::process::exit(1);
    } else {
        println!("{}", "Sucessfully added files".green());
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
            "ERROR: Failed comitting changes: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!(
            "{}",
            format!("Successfully committed with message: '{}'", commit_message).green()
        );
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
                "ERROR: Error pushing changes: {}",
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            eprintln!("{}", "Successfully pushed to remote! 🎉".green());
        }
    } else if push_choice.trim().to_lowercase() == "n" {
        println!("Not pushing to remote");
        println!("Run <gii push> to push origin/main");
    } else {
        eprintln!("{}", "ERROR: Invalid input".red());
        println!("Trying again...");
        push_to_origin();
    }
}

pub fn add_remote_origin(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", "ERROR: Invalid command".red());
        println!("Check usage with gii --h ar");
    } else {
        let url = &args[2];
        let output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(url)
            .output()
            .expect("Failed to execute git remote add origin");
        if !output.status.success() {
            eprintln!(
                "ERROR: Failed adding remote origin: {}",
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            println!("{}", "Remote origin added successfully. 🎉".green());
        }
    }
}

pub fn modify_remote_origin(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", "ERROR: Invalid command".red());
        println!("Check usage with gii --h mr");
    } else {
        let url = &args[2];
        let output = Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg("origin")
            .arg(url)
            .output()
            .expect("Failed to execute git remote set-url origin");
        if !output.status.success() {
            eprintln!(
                "ERROR: Failed adding remote origin: {}",
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            println!("{}", "Remote origin changed successfully. 🎉".green());
        }
    }
}

pub fn push() {
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push");
    if !output.status.success() {
        eprintln!(
            "ERROR: Error pushing changes: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", "Successfully pushed to remote! 🎉".green());
    }
}

pub fn pull() {
    let output = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git pull");
    if !output.status.success() {
        eprintln!(
            "ERROR: Error pulling changes: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", "Successfully pulled origin/main! 🎉".green());
    }
}

pub fn fetch() {
    let output = Command::new("git")
        .arg("fetch")
        .output()
        .expect("Failed to execute git pull");
    if !output.status.success() {
        eprintln!(
            "ERROR: Error fetching remote: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", "Successfully fetched origin/main! 🎉".green());
        println!("{}", String::from_utf8_lossy(&output.stdout).green());
    }
}

pub fn status() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status");
    if !output.status.success() {
        eprintln!(
            "ERROR: Failed to execute git status: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout).green());
    }
}

pub fn log() {
    let output = Command::new("git")
        .arg("log")
        .output()
        .expect("Failed to execute git log");
    if !output.status.success() {
        eprintln!("ERROR: {}", String::from_utf8_lossy(&output.stderr).red());
        std::process::exit(1);
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout).yellow());
    }
}

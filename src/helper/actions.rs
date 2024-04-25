use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub fn status() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status");
    if !output.status.success() {
        eprintln!("ERROR: {}", String::from_utf8_lossy(&output.stderr).red());
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
        println!("{}", String::from_utf8_lossy(&output.stdout).green());
    }
}

pub fn staging() {
    println!("");
    println!("Staging options:");
    println!("1. All files");
    println!("2. Specify which files");

    let mut choice = String::new();
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => add_all_files(),
        "2" => add_specific_files(),
        _ => {
            eprintln!("{}", "Invalid choice".red());
            staging();
        }
    }
}

pub fn add_all_files() {
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add .");
    if !output.status.success() {
        eprintln!(
            "Error adding files: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        eprintln!("{}", "Succuessfully added all files".green());
    }
}

pub fn add_specific_files() {
    print!("Enter the file name(s) (separated by spaces): ");
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
                "Error adding file: {}",
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            eprintln!(
                "{}",
                format!("Sucessfully added file: {}", file_name).green()
            );
        }
    }
}

pub fn commit_changes(commit_message: &str) {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to execute git commit");
    if !output.status.success() {
        eprintln!(
            "Error comitting changes: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        eprintln!(
            "{}",
            format!("Successfully committed with message: '{}'", commit_message).green()
        );
    }
}

pub fn push_to_origin() {
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
                String::from_utf8_lossy(&output.stderr).red()
            );
            std::process::exit(1);
        } else {
            eprintln!("{}", "Successfully pushed to remote! 🎉".green());
        }
    } else if push_choice.trim().to_lowercase() == "n" {
        eprintln!("Not pushing to remote");
    } else {
        eprintln!("Invalid input");
        push_to_origin();
    }
}

pub fn add_remote_origin(url: &str) {
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
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        eprintln!("{}", "Remote origin added successfully. 🎉".green());
    }
}

pub fn modify_remote_origin(url: &str) {
    let output = Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(url)
        .output()
        .expect("Failed to execute git remote set-url origin");
    if !output.status.success() {
        eprintln!(
            "Error adding remote origin: {}",
            String::from_utf8_lossy(&output.stderr).red()
        );
        std::process::exit(1);
    } else {
        eprintln!("{}", "Remote origin changed successfully. 🎉".green());
    }
}

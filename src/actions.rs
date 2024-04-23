use std::io::{self, Write};
use std::process::Command;

pub fn add_all_files() {
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

pub fn add_specific_file() {
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

pub fn commit_changes(commit_message: &str) {
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
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        eprintln!("Remote origin added successfully.");
    }
}

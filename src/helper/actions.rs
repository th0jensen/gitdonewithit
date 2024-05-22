use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn commit_push(args: &[String]) {
    if args.is_empty() {
        eprintln!("{}", Red.paint("ERROR: Invalid command"));
        println!("Check usage with gii help cp");
    } else {
        let commit_message = &args[0];
        staging();
        commit_changes(commit_message);
        push_to_origin();
    }
}

fn staging() {
    println!("");
    println!("{}:", Style::new().underline().paint("Staging options"));
    println!("1. All files");
    println!("2. Specify which files");
    println!("3. Interactive mode");

    let mut choice = String::new();
    println!("");
    print!(
        "{}: ",
        Style::new()
            .underline()
            .paint("Enter your choice ( 1 / 2 / 3 )")
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => add_all_files(),
        "2" => add_specific_files(),
        "3" => interactive_mode(),
        _ => {
            eprintln!("{}", Red.paint("ERROR: Invalid choice"));
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
            "{}: {}",
            Red.paint("ERROR: Failed adding files"),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!("{}", Green.paint("Successfully added all files"),);
    }
}

fn add_specific_files() {
    println!("");
    println!(
        "{}: ",
        Style::new()
            .underline()
            .paint("Enter the file path(s) (separated by spaces)")
    );
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
                "{} {}",
                Red.paint("ERROR: Failed adding file(s): "),
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        } else {
            println!("{} {}", Green.paint("Successfully added file: "), file_name);
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
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Failed adding files: "),
            status.to_string()
        );
        std::process::exit(1);
    } else {
        println!("{}", Green.paint("Successfully added files"),);
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
            "{} {}",
            Red.paint("ERROR: Failed committing changes: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!(
            "{} {}",
            Green.paint("Successfully committed with message: "),
            commit_message
        );
    }
}

fn push_to_origin() {
    println!("");
    print!(
        "{}: ",
        Style::new()
            .underline()
            .paint("Do you want to push to origin main? (y/n)")
    );
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
                "{} {}",
                Red.paint("ERROR: Failed pushing changes: "),
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        } else {
            println!("{}", Green.paint("Successfully pushed to remote! 🎉"));
        }
    } else if push_choice.trim().to_lowercase() == "n" {
        println!("Not pushing to remote");
        println!("Run <gii push> to push origin/main");
    } else {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Invalid input"),
            "Trying again..."
        );
        push_to_origin();
    }
}

pub fn add_remote_origin(args: &[String]) {
    if args.is_empty() {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Invalid command"),
            "Check usage with gii help ar"
        );
    } else {
        let url = &args[0];
        let output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(url)
            .output()
            .expect("Failed to execute git remote add origin");
        if !output.status.success() {
            eprintln!(
                "{} {}",
                Red.paint("ERROR: Failed adding remote origin: "),
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        } else {
            println!("{}", Green.paint("Remote origin added successfully! 🎉"));
        }
    }
}

pub fn modify_remote_origin(args: &[String]) {
    if args.is_empty() {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Invalid command: "),
            "Check usage with gii help mr"
        );
    } else {
        let url = &args[0];
        let output = Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg("origin")
            .arg(url)
            .output()
            .expect("Failed to execute git remote set-url origin");
        if !output.status.success() {
            eprintln!(
                "{} {}",
                Red.paint("ERROR: Failed adding remote origin: "),
                String::from_utf8_lossy(&output.stderr)
            );
            std::process::exit(1);
        } else {
            println!("{}", Green.paint("Remote origin changed successfully! 🎉"));
        }
    }
}

pub fn pull_rebase() {
    let output = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg("main")
        .arg("--rebase")
        .output()
        .expect("Failed to execute git pull --rebase");
    if !output.status.success() {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Failed pulling changes and rebasing: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!(
            "{}",
            Green.paint(
                "Succesfully pulled origin/main and rebased, your working tree is clean! 🎉"
            )
        );
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
            "{} {}",
            Red.paint("ERROR: Failed pulling changes: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!("{}", Green.paint("Succesfully pulled origin/main 🎉"));
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
            "{} {}",
            Red.paint("ERROR: Pushing changes: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!("{}", Green.paint("Successfully pushed to remote! 🎉"));
    }
}

pub fn fetch() {
    let output = Command::new("git")
        .arg("fetch")
        .output()
        .expect("Failed to execute git pull");
    if !output.status.success() {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Failed fetching remote: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!("{}", Green.paint("Successfully fetched origin/main 🎉"));
    }
}

pub fn status() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status");
    if !output.status.success() {
        eprintln!(
            "{} {}",
            Red.paint("ERROR: Failed to check repo status: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        println!("{}", Yellow.paint(String::from_utf8_lossy(&output.stdout)));
    }
}

pub fn log() {
    let output = Command::new("git")
        .arg("log")
        .output()
        .expect("Failed to execute git log");

    if !output.status.success() {
        println!(
            "{} {}",
            Red.paint("ERROR: Failed to fetch log: "),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    } else {
        let mut child = Command::new("nvim")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to launch Vim");

        child
            .stdin
            .as_mut()
            .expect("Failed to get Vim's stdin")
            .write_all(&output.stdout)
            .expect("Failed to write to Vim's stdin");

        let output = child.wait_with_output().expect("Failed to wait for Vim");
        if output.status.success() {
            println!("");
        } else {
            println!("Failed to launch neovim");
        }
    }
}

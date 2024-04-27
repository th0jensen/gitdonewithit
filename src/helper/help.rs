use ansi_term::{Colour, Style};

struct Command {
    name: &'static str,
    long_name: &'static str,
    arguments: &'static str,
    desc: &'static str,
}
pub fn help_message(args: &[String]) {
    let commands = [
        Command {
            name: "cp",
            long_name: "--commit-push",
            arguments: "[\"commit message\"]",
            desc: "commits files and pushes to remote",
        },
        Command {
            name: "ar",
            long_name: "--add-remote",
            arguments: "[url]",
            desc: "adds a remote repository to the repo",
        },
        Command {
            name: "mr",
            long_name: "--modify-remote",
            arguments: "[url]",
            desc: "changes the remote url of the repo",
        },
        Command {
            name: "push",
            long_name: "--p",
            arguments: "",
            desc: "pushes changes to origin/main",
        },
        Command {
            name: "pull",
            long_name: "--pl",
            arguments: "",
            desc: "pulls changes from origin/main",
        },
        Command {
            name: "fetch",
            long_name: "--f",
            arguments: "",
            desc: "fetches changes from origin/main",
        },
        Command {
            name: "status",
            long_name: "--s",
            arguments: "",
            desc: "prints the status of the current repo",
        },
        Command {
            name: "log",
            long_name: "--l",
            arguments: "",
            desc: "prints the commit log of the current repo",
        },
        Command {
            name: "help",
            long_name: "--h",
            arguments: "",
            desc: "prints this screen",
        },
        Command {
            name: "version",
            long_name: "-v",
            arguments: "",
            desc: "displays the current version",
        },
    ];

    if args.is_empty() {
        default_text();
        for command in commands.iter() {
            println!(
                "    {}, {} {} {} {}",
                Style::new().bold().paint(command.name),
                Style::new().bold().paint(command.long_name),
                command.arguments,
                Style::new().bold().paint("=>"),
                Style::new().italic().paint(command.desc)
            );

            println!("");
        }

        println!(
            "{}",
            Style::new()
                .underline()
                .paint("                             ")
        );
        println!(
            "{}",
            Style::new()
                .bold()
                .paint("    Use gii help <command> for more info on that specific command.")
        );
        println!(
            "{}",
            Style::new()
                .bold()
                .paint("    Remember that you can use this command to fuzzy find commands.")
        );
    } else {
        if !commands.iter().any(|command| {
            command.name.contains(&args[0])
                || command.long_name.contains(&args[0])
                || command.arguments.contains(&args[0])
                || command.desc.contains(&args[0])
        }) {
            eprintln!("{}", Colour::Red.paint("ERROR: Invalid command"));
            println!("Make sure to enter a valid command.");
            println!("Check <gii help> for usage");
        } else {
            default_text();
            for command in commands.iter() {
                if command.name.contains(&args[0])
                    | command.long_name.contains(&args[0])
                    | command.arguments.contains(&args[0])
                    | command.desc.contains(&args[0])
                {
                    println!(
                        "    {}, {} {} {} {}",
                        Style::new().bold().paint(command.name),
                        Style::new().bold().paint(command.long_name),
                        command.arguments,
                        Style::new().bold().paint("=>"),
                        Style::new().italic().paint(command.desc)
                    );
                    println!("");
                }
            }
        }
    }
}

fn default_text() {
    println!(" ");
    println!(
        "{}",
        Style::new()
            .underline()
            .paint("    gii <command> <arguments>")
    );
    println!(" ");
}

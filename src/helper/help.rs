struct Command {
    name: &'static str,
    arguments: &'static str,
    desc: &'static str,
}
pub fn help_message(args: &[String]) {
    let commands = [
        Command {
            name: "cp, --commit-push",
            arguments: "[\"commit message\"]",
            desc: "commits files and pushes to remote",
        },
        Command {
            name: "ar, --add-remote",
            arguments: "[url]",
            desc: "adds a remote repository to the repo",
        },
        Command {
            name: "mr, --modify-remote",
            arguments: "[url]",
            desc: "changes the remote url of the repo",
        },
        Command {
            name: "push, --p",
            arguments: "",
            desc: "pushes changes to origin/main",
        },
        Command {
            name: "pull, --pl",
            arguments: "",
            desc: "pulls changes from origin/main",
        },
        Command {
            name: "fetch, --f",
            arguments: "",
            desc: "fetches changes from origin/main",
        },
        Command {
            name: "status, --s",
            arguments: "",
            desc: "prints the status of the current repo",
        },
        Command {
            name: "log, --l",
            arguments: "",
            desc: "prints the commit log of the current repo",
        },
        Command {
            name: "help, --h",
            arguments: "",
            desc: "prints this screen",
        },
        Command {
            name: "version, --v",
            arguments: "",
            desc: "displays the current version",
        },
    ];

    if args.is_empty() {
        default_text();
        for command in commands.iter() {
            println!(
                "    {} {} => {}",
                command.name, command.arguments, command.desc
            );
            println!("");
        }
        println!("    ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄");
        println!("    Use gii --h <command> for more info on that specific command.");
    } else {
        default_text();
        for command in commands.iter() {
            if command.name.contains(&args[0]) {
                println!(
                    "    {} {} => {}",
                    command.name, command.arguments, command.desc
                );
                println!("");
            }
        }
    }
}

fn default_text() {
    println!(" ");
    println!("    gii <command> <arguments>");
    println!("    ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄");
}

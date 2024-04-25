struct Command {
    name: &'static str,
    arguments: &'static str,
    desc: &'static str,
}

pub fn help_message() {
    let commands = [
        Command {
            name: "cp",
            arguments: "[\"commit message\"]",
            desc: "commits files and pushes to remote.",
        },
        Command {
            name: "ar",
            arguments: "[url]",
            desc: "adds a remote repository to the repo.",
        },
        Command {
            name: "mr",
            arguments: "[url]",
            desc: "changes the remote url of the repo.",
        },
        Command {
            name: "status",
            arguments: "",
            desc: "prints the status of the current repo.",
        },
        Command {
            name: "log",
            arguments: "",
            desc: "prints the commit log of the current repo.",
        },
        Command {
            name: "help",
            arguments: "",
            desc: "prints this screen",
        },
        Command {
            name: "version",
            arguments: "",
            desc: "displays the current version",
        },
    ];

    println!(" ");
    println!("    gii <command> <arguments>");
    println!("    ....................................");

    for command in commands.iter() {
        println!("    {} {}", command.name, command.arguments);
        println!("    {}", command.desc);
        println!("    ....................................");
    }
}

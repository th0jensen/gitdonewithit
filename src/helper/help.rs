pub fn help_message() {
    let commands = [
        (
            "cp 'commit message'",
            "Add a commit message as an argument. Commits files and pushes to remote.",
        ),
        (
            "ar [url]",
            "Add a remote url as an argument. Adds a remote repository to the repo.",
        ),
        (
            "mr [url]",
            "Add a remote url as an argument. Changes the remote URL of the repo.",
        ),
        ("status", "Prints the status of the current repo."),
        ("log", "Prints the commit log of the current repo."),
        ("help", "Prints this screen"),
    ];

    println!(" ");
    println!("    gii <command> <arguments>");
    println!("    ....................................");

    for (command, description) in commands.iter() {
        println!("    {}", command);
        println!("    {}", description);
        println!("    ....................................");
    }
}

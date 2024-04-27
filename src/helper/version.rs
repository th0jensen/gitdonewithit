use ansi_term::Style;

pub fn version_message() {
    println!(" ");
    println!(
        "{}{}{}",
        Style::new().underline().bold().paint("gitdonewithit"),
        Style::new().underline().bold().paint(" "),
        Style::new().underline().paint("(gii)")
    );
    println!("{}", Style::new().italic().paint("Made by th0jensen"));
    println!(
        "Version: {}",
        Style::new()
            .bold()
            .italic()
            .paint(env!("CARGO_PKG_VERSION"))
    );
}

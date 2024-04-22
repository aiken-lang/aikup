mod cli;
mod cmd;

pub use cli::Cli;
use indoc::indoc;

pub const BANNER: &str = indoc! {
r#"================================================================================

    ░█▀▀▄░▀█▀░▒█░▄▀░▒█▀▀▀░▒█▄░▒█             Modern and modular toolkit
    ▒█▄▄█░▒█░░▒█▀▄░░▒█▀▀▀░▒█▒█▒█       for Cardano Smart Contract development.
    ▒█░▒█░▄█▄░▒█░▒█░▒█▄▄▄░▒█░░▀█                 Written in Rust.

    ================================================================================

    Repo       : https://github.com/aiken-lang/aiken
    Docs       : https://aiken-lang.org/
    Chat       : https://discord.gg/Vc3x8N9nz2
    Contribute : https://github.com/aiken-lang/aiken/blob/main/CONTRIBUTING.md

    ================================================================================"#
};

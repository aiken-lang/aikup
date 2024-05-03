mod cli;
mod cmd;
mod colors;
mod ctx;
mod utils;

pub use cli::Cli;

pub const BANNER: &str = color_print::cstr! {
r#"

 ░█▀▀▄░▀█▀░▒█░▄▀░▒█▀▀▀░▒█▄░▒█             Modern and modular toolkit
 ▒█▄▄█░▒█░░▒█▀▄░░▒█▀▀▀░▒█▒█▒█       for <green><bold>Cardano</bold></green> Smart Contract development.
 ▒█░▒█░▄█▄░▒█░▒█░▒█▄▄▄░▒█░░▀█                  Written in Rust.

 <magenta>repo:</magenta> <blue><italic><dim>https://github.com/aiken-lang/aiken</dim></italic></blue>
 <magenta>docs:</magenta> <blue><italic><dim>https://aiken-lang.org</dim></italic></blue>
 <magenta>chat:</magenta> <blue><italic><dim>https://discord.gg/Vc3x8N9nz2</dim></italic></blue>
 <magenta>contribute:</magenta> <blue><italic><dim>https://github.com/aiken-lang/aiken/blob/main/CONTRIBUTING.md</dim></italic></blue>"#
};

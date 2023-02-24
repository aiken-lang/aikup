use clap::Parser;

mod cmd;

/// aikup
#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
// #[clap(setting(clap::AppSettings::DeriveDisplayOrder))]
enum Cmd {
    Install(cmd::install::Args),
}

fn main() -> miette::Result<()> {
    match Cmd::parse() {
        Cmd::Install(args) => cmd::install::exec(args),
    }
}

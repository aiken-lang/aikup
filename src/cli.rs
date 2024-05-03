use crate::{cmd, BANNER};

use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = Some(BANNER))]
#[clap(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<cmd::Cmd>,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    pub async fn exec(self) -> miette::Result<()> {
        match self.cmd {
            Some(cmd) => cmd.exec().await,
            None => {
                println!("\n {}\n", BANNER.trim_start());

                cmd::install::latest().await
            }
        }
    }
}

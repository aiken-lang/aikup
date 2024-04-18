use crate::cmd;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
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
            None => install_latest().await,
        }
    }
}

async fn install_latest() -> miette::Result<()> {
    cmd::install::exec(cmd::install::Args::latest()).await
}

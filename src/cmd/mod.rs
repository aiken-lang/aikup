pub mod install;
pub mod list;

#[derive(clap::Subcommand)]
pub enum Cmd {
    Install(install::Args),
    List(list::Args),
}

impl Cmd {
    pub async fn exec(self) -> miette::Result<()> {
        match self {
            Cmd::Install(args) => args.exec().await,
            Cmd::List(args) => args.exec().await,
        }
    }
}

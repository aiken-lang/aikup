pub mod clean;
pub mod current;
pub mod install;
pub mod list;

#[derive(clap::Subcommand)]
pub enum Cmd {
    Clean(clean::Args),
    Current(current::Args),
    Install(install::Args),
    List(list::Args),
}

impl Cmd {
    pub async fn exec(self) -> miette::Result<()> {
        match self {
            Cmd::Clean(args) => args.exec().await,
            Cmd::Current(args) => args.exec().await,
            Cmd::Install(args) => args.exec().await,
            Cmd::List(args) => args.exec().await,
        }
    }
}

/// Install aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// Version to install
    #[clap(default_value = "latest")]
    release: String,
}

impl Args {
    pub fn latest() -> Self {
        Self {
            release: "latest".to_string(),
        }
    }
}

pub async fn exec(args: Args) -> miette::Result<()> {
    println!("{}", args.release);

    Ok(())
}

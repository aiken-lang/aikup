use miette::IntoDiagnostic;

use crate::utils::root_dir;

/// List available aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// show installed versions
    #[clap(short, long, default_value = "false")]
    installed: bool,
}

pub async fn exec(args: Args) -> miette::Result<()> {
    if args.installed {
        let aiken_root = root_dir()?;
        let versions_dir = aiken_root.join("versions");

        let mut entries = tokio::fs::read_dir(&versions_dir).await.into_diagnostic()?;

        println!("aikup: installed versions");

        while let Some(entry) = entries.next_entry().await.into_diagnostic()? {
            let path = entry.path();

            if path.is_dir() {
                println!("{}", path.file_name().unwrap().to_string_lossy());
            }
        }
    } else {
        let octocrab = octocrab::instance();

        let releases = octocrab
            .repos("aiken-lang", "aiken")
            .releases()
            .list()
            .send()
            .await
            .into_diagnostic()?;

        println!("aikup: available versions");

        for release in releases {
            println!("{}", release.tag_name);
        }
    }

    Ok(())
}

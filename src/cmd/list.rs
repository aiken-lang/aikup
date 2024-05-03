use miette::IntoDiagnostic;

use crate::{ctx, utils::root_dir};

/// List available aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// show installed versions
    #[clap(short, long, default_value = "false")]
    installed: bool,
}

impl Args {
    pub async fn exec(self) -> miette::Result<()> {
        let ctx = ctx::instance();

        if self.installed {
            let aiken_root = root_dir()?;
            let versions_dir = aiken_root.join("versions");

            let mut entries = tokio::fs::read_dir(&versions_dir).await.into_diagnostic()?;

            println!(
                "{} {}\n",
                ctx.aikup_label(),
                ctx.colors.info_text("installed versions")
            );

            while let Some(entry) = entries.next_entry().await.into_diagnostic()? {
                let path = entry.path();

                if path.is_dir() {
                    let display_name = path.file_name().unwrap().to_string_lossy();

                    println!("{}", ctx.colors.version_text(display_name).bold());
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

            println!(
                "{} {}\n",
                ctx.aikup_label(),
                ctx.colors.info_text("available versions")
            );

            for release in releases {
                println!("{}", ctx.colors.version_text(&release.tag_name).bold());
            }
        }

        Ok(())
    }
}

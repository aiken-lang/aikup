use miette::IntoDiagnostic;

use crate::{
    ctx,
    utils::{read_parent_name_from_link, remove_file_if_exists, root_dir},
};

/// Remove all aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// keep the version currently in use
    #[clap(short, long, default_value = "false")]
    keep_current: bool,
}

impl Args {
    pub async fn exec(self) -> miette::Result<()> {
        let ctx = ctx::instance();

        let aiken_root = root_dir()?;
        let versions_dir = aiken_root.join("versions");

        let bin_dir = aiken_root.join("bin");
        let sym_bin = bin_dir.join("aiken");

        let mut entries = tokio::fs::read_dir(&versions_dir).await.into_diagnostic()?;

        println!(
            "{} {}",
            ctx.aikup_label(),
            ctx.colors.info_text("removing versions")
        );

        let (_, current_version) = read_parent_name_from_link(&sym_bin)
            .await
            .unwrap_or_default();

        if !self.keep_current {
            remove_file_if_exists(&sym_bin).await?;
        }

        while let Some(entry) = entries.next_entry().await.into_diagnostic()? {
            let path = entry.path();

            let display_name = path
                .file_name()
                .and_then(|l| l.to_str())
                .map(|l| l.to_string())
                .unwrap_or_default();

            let keep_entry = self.keep_current && display_name == current_version;

            if !keep_entry && path.is_dir() {
                tokio::fs::remove_dir_all(&path).await.into_diagnostic()?;

                println!(
                    "{} {} {}",
                    ctx.aikup_label(),
                    ctx.colors.error_text("removed"),
                    ctx.colors.version_text(display_name).italic().dim()
                );
            } else if path.is_file() {
                tokio::fs::remove_file(path).await.into_diagnostic()?;
            } else {
                println!(
                    "{} {} {}",
                    ctx.aikup_label(),
                    ctx.colors.success_text("kept"),
                    ctx.colors.version_text(display_name).italic().dim()
                );
            }
        }

        Ok(())
    }
}

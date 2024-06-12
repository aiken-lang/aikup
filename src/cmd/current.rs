use crate::{
    ctx,
    utils::{read_parent_name_from_link, root_dir},
};

/// Show the aiken version currently in use
#[derive(clap::Args)]
pub struct Args {}

impl Args {
    pub async fn exec(self) -> miette::Result<()> {
        let ctx = ctx::instance();

        let bin_dir = root_dir()?.join("bin");

        #[cfg(unix)]
        let sym_bin = bin_dir.join("aiken");

        #[cfg(unix)]
        let current_version = read_parent_name_from_link(&sym_bin).await;

        #[cfg(windows)]
        let current_version = tokio::fs::read_to_string(bin_dir.join("current"))
            .await
            .ok()
            .map(|v| (bin_dir.join("current"), v));

        match current_version {
            Some((path, current_version)) if path.exists() && !current_version.is_empty() => {
                println!(
                    "{} {} {}",
                    ctx.aikup_label(),
                    ctx.colors.info_text("currently using"),
                    ctx.colors.version_text(current_version).italic().dim()
                );
            }
            Some(_) | None => {
                super::install::latest().await?;
            }
        }

        Ok(())
    }
}

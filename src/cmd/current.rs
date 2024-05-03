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

        let sym_bin = root_dir()?.join("bin").join("aiken");

        let current_version = read_parent_name_from_link(&sym_bin).await;

        match current_version {
            Some((path, current_version)) if path.exists() => {
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

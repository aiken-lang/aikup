use std::env;

#[cfg(unix)]
use tokio::fs::symlink;

#[cfg(windows)]
use tokio::fs::symlink_file as symlink;

use flate2::read::GzDecoder;
use http_body_util::BodyExt;
use miette::IntoDiagnostic;
use tar::Archive;

use crate::{
    ctx,
    utils::{create_dir_all_if_not_exists, remove_file_if_exists, root_dir},
};

/// Install aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// version to install
    release: Option<String>,
    /// do not switch to the installed version
    #[clap(short, long, default_value = "false")]
    no_switch: bool,
}

impl Args {
    pub fn latest() -> Self {
        Self {
            release: None,
            no_switch: false,
        }
    }

    pub async fn exec(self) -> miette::Result<()> {
        let ctx = ctx::instance();
        let octocrab = octocrab::instance();

        let aiken_root = root_dir()?;

        let bin_dir = aiken_root.join("bin");
        let versions_dir = aiken_root.join("versions");

        let release = match self.release {
            Some(tag) => {
                println!(
                    "{} {} {}",
                    ctx.aikup_label(),
                    ctx.colors.info_text("installing"),
                    ctx.colors.version_text(&tag).italic().dim()
                );

                octocrab
                    .repos("aiken-lang", "aiken")
                    .releases()
                    .get_by_tag(&tag)
                    .await
                    .into_diagnostic()?
            }
            None => {
                println!(
                    "{} {} {}",
                    ctx.aikup_label(),
                    ctx.colors.warning_text("no version specified;"),
                    ctx.colors.info_text("installing latest"),
                );

                octocrab
                    .repos("aiken-lang", "aiken")
                    .releases()
                    .get_latest()
                    .await
                    .into_diagnostic()?
            }
        };

        let asset_name = asset_name(&release.tag_name);

        let search_result = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name);

        let Some(asset) = search_result else {
            miette::bail!("{} no release found for {}", ctx.aikup_label(), asset_name);
        };

        let install_dir = versions_dir.join(&release.tag_name);
        let src_bin = install_dir.join("aiken");

        if src_bin.try_exists().into_diagnostic()? {
            println!(
                "{} {} {}",
                ctx.aikup_label(),
                ctx.colors.warning_text("already installed"),
                ctx.colors.version_text(&release.tag_name).italic().dim()
            );
        } else {
            println!(
                "{} {} {}",
                ctx.aikup_label(),
                ctx.colors.info_text("downloading"),
                ctx.colors.version_text(&release.tag_name).italic().dim()
            );

            let bytes = octocrab
                ._get(asset.browser_download_url.to_string())
                .await
                .into_diagnostic()?
                .into_body()
                .collect()
                .await
                .into_diagnostic()?
                .to_bytes();

            let decoder = GzDecoder::new(&bytes[..]);

            let mut archive = Archive::new(decoder);

            let install_dir = versions_dir.join(&release.tag_name);

            create_dir_all_if_not_exists(&versions_dir).await?;

            archive.unpack(&install_dir).into_diagnostic()?;

            println!(
                "{} {} {}",
                ctx.aikup_label(),
                ctx.colors.success_text("installed"),
                ctx.colors.version_text(&release.tag_name).italic().dim()
            );
        }

        if !self.no_switch {
            let sym_bin = bin_dir.join("aiken");
            let src_bin = install_dir.join("aiken");

            match tokio::fs::read_link(&sym_bin).await {
                Ok(real_path) if real_path == src_bin => {
                    println!(
                        "{} {} {}",
                        ctx.aikup_label(),
                        ctx.colors.warning_text("already switched"),
                        ctx.colors.version_text(&release.tag_name).italic().dim()
                    );
                }
                Ok(_) | Err(_) => {
                    create_dir_all_if_not_exists(&bin_dir).await?;

                    remove_file_if_exists(&sym_bin).await?;

                    symlink(src_bin, sym_bin).await.into_diagnostic()?;

                    println!(
                        "{} {} {}",
                        ctx.aikup_label(),
                        ctx.colors.success_text("switched"),
                        ctx.colors.version_text(&release.tag_name).italic().dim()
                    );
                }
            }
        }

        Ok(())
    }
}

pub async fn latest() -> miette::Result<()> {
    Args::latest().exec().await
}

fn asset_name(tag_name: &str) -> String {
    let os = match env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        os => os,
    };

    let arch = match env::consts::ARCH {
        "x86" => "amd64",
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        arch => arch,
    };

    format!("aiken_{}_{}_{}.tar.gz", tag_name, os, arch)
}

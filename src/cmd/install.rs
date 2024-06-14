use std::env;

use flate2::read::GzDecoder;
use http_body_util::BodyExt;
use miette::IntoDiagnostic;
use semver::Version;
use tar::Archive;
use which::which;

use crate::{
    ctx,
    partial_config::PartialConfig,
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
                let current_dir = env::current_dir().into_diagnostic()?;

                let opt_config = PartialConfig::load(&current_dir).await.ok();

                if let Some(config) = opt_config {
                    let version = format!("v{}", &config.compiler);

                    println!(
                        "{} {} {}",
                        ctx.aikup_label(),
                        ctx.colors.info_text("detected"),
                        ctx.colors.version_text(&version).italic().dim()
                    );

                    octocrab
                        .repos("aiken-lang", "aiken")
                        .releases()
                        .get_by_tag(&version)
                        .await
                        .into_diagnostic()?
                } else {
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
            }
        };

        let version = Version::parse(&release.tag_name.replace('v', "")).into_diagnostic()?;

        let cut_off = Version::parse("1.0.26-alpha").into_diagnostic()?;

        let is_past_cut_off = version > cut_off;

        let asset_name = asset_name(&release.tag_name, is_past_cut_off)?;

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
            #[cfg(unix)]
            {
                let sym_bin = bin_dir.join("aiken");

                let src_bin = if is_past_cut_off {
                    install_dir
                        .join(asset_name.replace(".tar.gz", ""))
                        .join("aiken")
                } else {
                    install_dir.join("aiken")
                };

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

                        tokio::fs::symlink(src_bin, sym_bin)
                            .await
                            .into_diagnostic()?;

                        println!(
                            "{} {} {}",
                            ctx.aikup_label(),
                            ctx.colors.success_text("switched"),
                            ctx.colors.version_text(&release.tag_name).italic().dim()
                        );
                    }
                }
            }

            #[cfg(windows)]
            {
                let sym_bin = bin_dir.join("aiken.exe");

                let current = bin_dir.join("current");

                let src_bin = if is_past_cut_off {
                    install_dir
                        .join(asset_name.replace(".tar.gz", ""))
                        .join("aiken.exe")
                } else {
                    install_dir.join("aiken.exe")
                };

                if tokio::fs::try_exists(&current).await.into_diagnostic()? {
                    let current_version = tokio::fs::read_to_string(&current)
                        .await
                        .into_diagnostic()?;

                    if current_version.trim() == release.tag_name {
                        println!(
                            "{} {} {}",
                            ctx.aikup_label(),
                            ctx.colors.warning_text("already switched"),
                            ctx.colors.version_text(&release.tag_name).italic().dim()
                        );
                    } else {
                        remove_file_if_exists(&sym_bin).await?;

                        tokio::fs::copy(src_bin, sym_bin).await.into_diagnostic()?;

                        tokio::fs::write(current, &release.tag_name)
                            .await
                            .into_diagnostic()?;

                        println!(
                            "{} {} {}",
                            ctx.aikup_label(),
                            ctx.colors.success_text("switched"),
                            ctx.colors.version_text(&release.tag_name).italic().dim()
                        );
                    }
                } else {
                    create_dir_all_if_not_exists(&bin_dir).await?;

                    remove_file_if_exists(&sym_bin).await?;

                    tokio::fs::copy(src_bin, sym_bin).await.into_diagnostic()?;

                    tokio::fs::write(current, &release.tag_name)
                        .await
                        .into_diagnostic()?;

                    println!(
                        "{} {} {}",
                        ctx.aikup_label(),
                        ctx.colors.success_text("switched"),
                        ctx.colors.version_text(&release.tag_name).italic().dim()
                    );
                }
            }
        }

        match which("aiken") {
            Ok(path) if path.display().to_string().contains(".aiken/bin") => (),
            Ok(_) => {
                println!(
                    "{} {}",
                    ctx.aikup_label(),
                    ctx.colors
                        .warning_text("aiken is in your PATH but not managed by this tool")
                );
            }
            Err(_) => {
                println!(
                    "{} {}",
                    ctx.aikup_label(),
                    ctx.colors.warning_text(format!(
                        "aiken not found in your $PATH, please add \"{}\" to your $PATH",
                        bin_dir.display()
                    ))
                );
            }
        }

        Ok(())
    }
}

pub async fn latest() -> miette::Result<()> {
    Args::latest().exec().await
}

fn asset_name(tag_name: &str, is_past_cut_off: bool) -> miette::Result<String> {
    if is_past_cut_off {
        let os = match env::consts::OS {
            "macos" => "apple-darwin",
            "windows" => "pc-windows-msvc",
            "linux" => "unknown-linux-gnu",
            os => os,
        };

        let arch = match env::consts::ARCH {
            "x86" => "x86_64",
            arch => arch,
        };

        let asset_name = format!("aiken-{}-{}.tar.gz", arch, os);

        Ok(asset_name)
    } else {
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

        let asset_name = format!("aiken_{}_{}_{}.tar.gz", tag_name, os, arch);

        Ok(asset_name)
    }
}

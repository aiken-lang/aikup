use std::env;

use flate2::read::GzDecoder;
use http_body_util::BodyExt;
use miette::IntoDiagnostic;
use tar::Archive;

use crate::BANNER;

/// Install aiken versions
#[derive(clap::Args)]
pub struct Args {
    /// Version to install
    release: Option<String>,
}

impl Args {
    pub fn latest() -> Self {
        Self { release: None }
    }
}

pub async fn exec(args: Args) -> miette::Result<()> {
    println!("{}", BANNER);

    let octocrab = octocrab::instance();

    let release = match args.release {
        Some(tag) => octocrab
            .repos("aiken-lang", "aiken")
            .releases()
            .get_by_tag(&tag)
            .await
            .into_diagnostic()?,
        None => {
            println!("aikup: no version specified; installing latest");

            octocrab
                .repos("aiken-lang", "aiken")
                .releases()
                .get_latest()
                .await
                .into_diagnostic()?
        }
    };

    println!("aikup: installing aiken ({})", release.tag_name);

    let asset_name = asset_name(&release.tag_name);

    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name == asset_name);

    match asset {
        Some(asset) => {
            println!("aikup: downloading aiken");

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

            archive.unpack(".").into_diagnostic()?;

            println!("aikup: aiken installed");
        }
        None => {
            eprintln!("aikup: no release found for {}", asset_name);

            std::process::exit(1);
        }
    }

    Ok(())
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

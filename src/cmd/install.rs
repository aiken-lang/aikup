use http_body_util::BodyExt;
use miette::IntoDiagnostic;

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

    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name == "aiken_v1.0.26-alpha_darwin_amd64.tar.gz");

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

            println!("{:#?}", bytes);

            println!("aikup: aiken installed");
        }
        None => {
            eprintln!("aikup: no release found for your platform");

            std::process::exit(1);
        }
    }

    Ok(())
}

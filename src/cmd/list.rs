use miette::IntoDiagnostic;

/// List aiken versions
#[derive(clap::Args)]
pub struct Args {}

pub async fn exec(_args: Args) -> miette::Result<()> {
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

    Ok(())
}

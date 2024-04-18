#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = aikup::Cli::default();

    cli.exec().await
}

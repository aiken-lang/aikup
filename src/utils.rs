use std::path::PathBuf;

use miette::IntoDiagnostic;

pub async fn create_dir_all_if_not_exists(path: &PathBuf) -> miette::Result<()> {
    if !path.try_exists().into_diagnostic()? {
        tokio::fs::create_dir_all(path).await.into_diagnostic()?;
    }

    Ok(())
}

pub async fn remove_file_if_exists(path: &PathBuf) -> miette::Result<()> {
    if path.try_exists().into_diagnostic()? {
        tokio::fs::remove_file(path).await.into_diagnostic()?;
    }

    Ok(())
}

pub fn root_dir() -> miette::Result<PathBuf> {
    let Some(aiken_root) = dirs::home_dir().map(|home| home.join(".aiken")) else {
        miette::bail!("cannot find home directory")
    };

    Ok(aiken_root)
}

pub async fn read_parent_name_from_link(path: &PathBuf) -> Option<(PathBuf, String)> {
    tokio::fs::read_link(&path).await.ok().and_then(|link| {
        link.parent().and_then(|parent| {
            parent
                .file_name()
                .and_then(|l| l.to_str())
                .map(|l| (link.clone(), l.to_string()))
        })
    })
}

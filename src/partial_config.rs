use std::path::Path;

use miette::IntoDiagnostic;
use semver::Version;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct PartialConfig {
    #[serde(deserialize_with = "deserialize_version")]
    pub compiler: Version,
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?.replace('v', "");

    Version::parse(&buf).map_err(serde::de::Error::custom)
}

impl PartialConfig {
    pub async fn load(dir: &Path) -> miette::Result<PartialConfig> {
        let config_path = dir.join("aiken.toml");

        let raw_config = tokio::fs::read_to_string(&config_path)
            .await
            .into_diagnostic()?;

        let result: Self = toml::from_str(&raw_config).into_diagnostic()?;

        Ok(result)
    }
}

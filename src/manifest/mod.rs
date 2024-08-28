pub mod version;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
enum ManifestType {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

#[derive(Deserialize, Serialize, Debug)]
struct VersionManifest {
    id: String,
    #[serde(rename = "type")]
    type_: ManifestType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LastManifest {
    release: String,
    snapshot: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {
    latest: LastManifest,
    versions: Vec<VersionManifest>,
}

use crate::handlers::error::*;

impl Manifest {
    #[cfg(feature = "sync")]
    pub fn parse() -> Result<Self, Error> {
        Ok(serde_json::from_reader(reqwest::blocking::get(
            "https://launchermeta.mojang.com/mc/game/version_manifest.json",
        )?)?)
    }

    #[cfg(feature = "async")]
    pub async fn parse() -> Result<Self, Error> {
        Ok(serde_json::from_reader(
            reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
                .await?
                .bytes()
                .await?
                .as_ref(),
        )?)
    }

    pub fn parse_from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(serde_json::from_reader(std::fs::File::open(path)?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sync")]
    #[test]
    fn parse() {
        let manifest = Manifest::parse().unwrap();

        assert!(manifest.latest.snapshot != manifest.latest.release);
        assert!(manifest.versions.len() > 0);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn parse() {
        let manifest = Manifest::parse().await.unwrap();

        assert_ne!(manifest.latest.snapshot, manifest.latest.release);
        assert!(manifest.versions.len() > 0);
    }

    #[test]
    fn parse_from_file() {
        let manifest = Manifest::parse_from_file("manifest.json").unwrap();

        assert_ne!(manifest.latest.snapshot, manifest.latest.release);
        assert!(manifest.versions.len() > 0);

        assert_eq!(manifest.versions.first().unwrap().id, "24w35a");
    }
}

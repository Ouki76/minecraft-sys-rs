use super::VersionManifest;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: u32,
    #[serde(rename = "totalSize")]
    total_size: u32,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Download {
    sha1: String,
    size: u32,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Downloads {
    client: Download,
    client_mappings: Download,
    server: Download,
    server_mappings: Download,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: u32,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LibDownload {
    artifact: Artifact,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Os {
    name: Option<String>,
    arch: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    action: String,
    os: Os,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Library {
    downloads: LibDownload,
    name: String,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientLoggingFile {
    id: String,
    sha1: String,
    size: u32,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientLogging {
    argument: String,
    file: ClientLoggingFile,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Logging {
    client: ClientLogging,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Version {
    arguments: serde_json::Value,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    assets: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: u32,
    downloads: Downloads,
    id: String,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
    logging: Logging,
    #[serde(rename = "mainClass")]
    main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: u32,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    #[serde(rename = "type")]
    type_: super::ManifestType,
}

impl VersionManifest {
    #[cfg(feature = "sync")]
    pub fn data(&self) -> Result<Version, crate::handlers::error::Error> {
        Ok(serde_json::from_reader(reqwest::blocking::get(&self.url)?)?)
    }

    #[cfg(feature = "async")]
    pub async fn data(&self) -> Result<Version, crate::handlers::error::Error> {
        Ok(serde_json::from_reader(
            reqwest::get(&self.url).await?.bytes().await?.as_ref(),
        )?)
    }

    pub fn data_from_file<P>(path: P) -> Result<Version, crate::handlers::error::Error>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(serde_json::from_reader(std::fs::File::open(path)?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Manifest, *};

    #[cfg(feature = "sync")]
    #[test]
    fn data() {
        let manifest = Manifest::parse().unwrap();

        let data = manifest.versions.first().unwrap().data().unwrap();

        assert!(data.libraries.len() > 0);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn data() {
        let manifest = Manifest::parse().await.unwrap();

        let data = manifest.versions.first().unwrap().data().await.unwrap();

        assert!(data.libraries.len() > 0);
    }

    #[test]
    fn data_from_file() {
        let data = VersionManifest::data_from_file("version_manifest.json").unwrap();

        assert!(data.libraries.len() > 0);
        assert_eq!(data.id, "1.21.1");
    }
}

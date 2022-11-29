use crate::hash_value::HashValue;
use crate::Error;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "manifest-version")]
    pub(crate) manifest_version: String,
    pub(crate) date: NaiveDate,
    pub(crate) profiles: HashMap<String, Vec<String>>,
    pub(crate) renames: HashMap<String, Rename>,
    pub(crate) artifacts: HashMap<String, Artifact>,
    #[serde(rename = "pkg")]
    pub(crate) packages: HashMap<String, Package>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub(crate) version: Option<String>,
    pub(crate) git_commit_hash: Option<HashValue>,
    #[serde(rename = "target")]
    pub(crate) targets: HashMap<String, PackageBuild>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageBuild {
    pub(crate) available: bool,
    #[serde(rename = "hash")]
    pub(crate) gz_hash: Option<HashValue>,
    #[serde(rename = "url")]
    pub(crate) gz_url: Option<String>,
    pub(crate) xz_hash: Option<HashValue>,
    pub(crate) xz_url: Option<String>,
    pub(crate) components: Option<Vec<Component>>,
    pub(crate) extensions: Option<Vec<Component>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    pub(crate) to: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "target")]
    pub(crate) targets: HashMap<String, Vec<ArtifactBuild>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactBuild {
    #[serde(rename = "hash-sha256")]
    pub(crate) hash_sha256: HashValue,
    pub(crate) url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "pkg")]
    pub(crate) package: String,
    pub(crate) target: String,
}

pub fn try_parse_manifest(string: &str) -> Result<Manifest, Error> {
    let parsed: Manifest = toml::from_str(string)?;
    Ok(parsed)
}

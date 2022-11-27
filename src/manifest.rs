use crate::hash_value::HashValue;
use crate::manifest_v2;
use crate::Error;
use chrono::NaiveDate;
use std::collections::HashMap;

const TARGET_INDEPENDENT_NAME: &str = "*";

#[derive(Clone, Debug)]
pub struct Manifest {
    version: String,
    date: NaiveDate,
    profiles: HashMap<String, Vec<String>>,
    renames: HashMap<String, String>,
    packages: HashMap<String, PackageBuilds>,
}

#[derive(Clone, Debug)]
pub struct PackageInfo {
    version: String,
    git_commit: HashValue,
}

#[derive(Clone, Debug)]
pub struct PackageBuilds {
    info: Option<PackageInfo>,
    artifacts: TargetMap<Option<PackageBuild>>,
}

#[derive(Clone, Debug)]
pub struct PackageBuild {
    artifacts: HashMap<Compression, RemoteBinary>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Digest {
    Sha256,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compression {
    Gzip,
    Xz,
}

#[derive(Clone, Debug)]
pub struct RemoteBinary {
    url: String,
    digests: HashMap<Digest, HashValue>,
}

type Target = String;

#[derive(Clone, Debug)]
pub enum TargetMap<V> {
    Independent(V),
    Dependent(HashMap<Target, V>),
}

impl Manifest {
    fn translate_build(from: &manifest_v2::PackageBuild) -> Option<PackageBuild> {
        if from.available {
            let mut artifacts = HashMap::new();
            for (compression, url, hash) in [
                (Compression::Gzip, &from.gz_url, &from.gz_hash),
                (Compression::Xz, &from.xz_url, &from.xz_hash),
            ] {
                if let (Some(url), Some(hash)) = (url, hash) {
                    artifacts.insert(
                        compression,
                        RemoteBinary {
                            url: url.to_string(),
                            digests: std::iter::once((Digest::Sha256, hash.clone())).collect(),
                        },
                    );
                }
            }
            let build = PackageBuild { artifacts };
            Some(build)
        } else {
            None
        }
    }

    fn from_v2(parsed: manifest_v2::Manifest) -> Result<Manifest, Error> {
        let mut packages = HashMap::with_capacity(parsed.packages.len());
        for (name, parsed_package) in parsed.packages {
            let version_info = match (parsed_package.version, parsed_package.git_commit_hash) {
                (Some(version), Some(git_commit)) => Some(PackageInfo {
                    version,
                    git_commit,
                }),
                _ => None,
            };
            let artifacts = if parsed_package.targets.len() == 1
                && parsed_package.targets.contains_key(TARGET_INDEPENDENT_NAME)
            {
                let build = parsed_package
                    .targets
                    .get(TARGET_INDEPENDENT_NAME)
                    .expect("Failed to extract target-independent package");
                TargetMap::Independent(Self::translate_build(build))
            } else {
                let mut artifacts = HashMap::with_capacity(parsed_package.targets.len());
                for (target_name, parsed_target) in parsed_package.targets {
                    if target_name == TARGET_INDEPENDENT_NAME {
                        return Err(Error::ConflictingTargetDependence(name));
                    }
                    artifacts.insert(target_name, Self::translate_build(&parsed_target));
                }
                TargetMap::Dependent(artifacts)
            };
            let builds = PackageBuilds {
                info: version_info,
                artifacts,
            };
            packages.insert(name, builds);
        }
        let renames: HashMap<String, String> = parsed
            .renames
            .into_iter()
            .map(|(from, rename)| (from, rename.to))
            .collect();
        let result = Manifest {
            version: parsed.manifest_version,
            date: parsed.date,
            profiles: parsed.profiles,
            renames,
            packages,
        };
        Ok(result)
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }

    pub fn get_profiles(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn get_components(&self, profile: &str) -> Option<Vec<String>> {
        self.profiles.get(profile).cloned()
    }
}

impl TryFrom<&str> for Manifest {
    type Error = Error;

    fn try_from(string: &str) -> Result<Manifest, Error> {
        let parsed = manifest_v2::try_parse_manifest(string)?;
        println!("{:#?}", parsed);
        Manifest::from_v2(parsed)
    }
}

use crate::hash_value::HashValue;
use crate::supported_target::{SupportedTarget, TARGET_INDEPENDENT_NAME};
use crate::{manifest_v2, Error};
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use target_lexicon::Triple;

#[derive(Clone, Debug)]
pub struct Manifest {
    version: String,
    date: NaiveDate,
    profiles: HashMap<String, Vec<String>>,
    _renames: HashMap<String, String>,
    packages: HashMap<String, PackageBuilds>,
    _components: HashMap<Triple, HashMap<(String, SupportedTarget), Component>>,
    component_name_map: HashMap<Triple, HashMap<String, (String, SupportedTarget)>>,
}

#[derive(Clone, Debug)]
pub struct InstallSpec {
    pub profile: String,
    pub components: HashSet<String>,
    pub targets: HashSet<String>,
}

#[derive(Clone, Debug)]
struct Component {
    _is_extension: bool,
}

#[derive(Clone, Debug)]
struct PackageInfo {
    version: String,
    git_commit: HashValue,
}

#[derive(Clone, Debug)]
struct PackageBuilds {
    name: String,
    info: Option<PackageInfo>,
    artifacts: TargetMap<Option<PackageBuild>>,
}

impl PackageBuilds {
    fn get(&self, supported_target: &SupportedTarget) -> Result<PackageBuild, Error> {
        match &self.artifacts {
            TargetMap::Independent(build) => {
                if let Some(build) = build {
                    Ok(build.clone())
                } else {
                    Err(Error::PackageUnavailable(self.name.clone(), supported_target.clone()))
                }
            }
            TargetMap::Dependent(map) => {
                let target = if let SupportedTarget::Dependent(target) = supported_target {
                    target
                } else {
                    return Err(Error::PackageNotTargetIndependent(self.name.clone()));
                };
                if let Some(build) = map.get(target) {
                    if let Some(build) = build {
                        Ok(build.clone())
                    } else {
                        Err(Error::PackageUnavailable(self.name.clone(), supported_target.clone()))
                    }
                } else {
                    Err(Error::PackageUnknown(self.name.clone(), supported_target.clone()))
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ManifestPackage {
    pub name: String,
    pub version: String,
    pub git_commit: HashValue,
    pub supported_target: SupportedTarget,
    pub tarballs: Vec<(Compression, RemoteBinary)>,
}

impl ManifestPackage {
    pub fn unique_identifier(&self) -> HashValue {
        // Let's just choose the largest hash for now
        let mut hashes: Vec<HashValue> = self
            .tarballs
            .iter()
            .flat_map(|(_, r)| &r.digests)
            .map(|(_, value)| value.clone())
            .collect();
        hashes.sort();
        hashes
            .last()
            .cloned()
            .unwrap_or_else(|| panic!("Missing digest for package {}", self.name.clone()))
    }
}

#[derive(Clone, Debug)]
struct PackageBuild {
    artifacts: HashMap<Compression, RemoteBinary>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Digest {
    Sha256,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compression {
    None,
    Gzip,
    Xz,
}

#[derive(Clone, Debug)]
pub struct RemoteBinary {
    pub url: String,
    pub digests: HashMap<Digest, HashValue>,
}

#[derive(Clone, Debug)]
enum TargetMap<V> {
    Independent(V),
    Dependent(HashMap<Triple, V>),
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
        for (name, parsed_package) in &parsed.packages {
            let version_info = match (&parsed_package.version, &parsed_package.git_commit_hash) {
                (Some(version), Some(git_commit)) => Some(PackageInfo {
                    version: version.to_string(),
                    git_commit: git_commit.clone(),
                }),
                _ => None,
            };
            let artifacts =
                if parsed_package.targets.len() == 1 && parsed_package.targets.contains_key(TARGET_INDEPENDENT_NAME) {
                    let build = parsed_package
                        .targets
                        .get(TARGET_INDEPENDENT_NAME)
                        .expect("Failed to extract target-independent package");
                    TargetMap::Independent(Self::translate_build(build))
                } else {
                    let mut artifacts: HashMap<Triple, _> = HashMap::with_capacity(parsed_package.targets.len());
                    for (target_name, parsed_target) in &parsed_package.targets {
                        if target_name == TARGET_INDEPENDENT_NAME {
                            return Err(Error::ConflictingTargetDependence(name.to_string()));
                        }
                        artifacts.insert(
                            Triple::from_str(target_name.as_str())?,
                            Self::translate_build(parsed_target),
                        );
                    }
                    TargetMap::Dependent(artifacts)
                };
            let builds = PackageBuilds {
                name: name.to_string(),
                info: version_info,
                artifacts,
            };
            packages.insert(name.to_string(), builds);
        }
        let mut components = HashMap::new();
        let rust = parsed.packages.get("rust").ok_or(Error::RustMissing)?;
        for (target, build) in &rust.targets {
            let mut target_components = HashMap::new();
            let parsed_components = &build.components;
            let parsed_extensions = &build.extensions;
            for (parsed_components, is_extension) in [(parsed_components, false), (parsed_extensions, true)] {
                for parsed_component in parsed_components.iter().flatten() {
                    let component = Component {
                        _is_extension: is_extension,
                    };
                    let package = parsed_component.package.to_string();
                    let component_target = SupportedTarget::from_str(parsed_component.target.as_str())?;
                    target_components.insert((package, component_target), component);
                }
            }
            components.insert(Triple::from_str(target.as_str())?, target_components);
        }
        let renames: HashMap<String, String> = parsed
            .renames
            .into_iter()
            .map(|(from, rename)| (from, rename.to))
            .collect();
        let component_name_map = Self::build_component_name_map(&components, &renames);
        let result = Manifest {
            version: parsed.manifest_version,
            date: parsed.date,
            profiles: parsed.profiles,
            _renames: renames,
            packages,
            _components: components,
            component_name_map,
        };
        Ok(result)
    }

    fn build_component_name_map(
        components: &HashMap<Triple, HashMap<(String, SupportedTarget), Component>>,
        renames: &HashMap<String, String>,
    ) -> HashMap<Triple, HashMap<String, (String, SupportedTarget)>> {
        let inverse_renames: HashMap<&str, &str> = renames.iter().map(|(k, v)| (v.as_str(), k.as_str())).collect();
        let mut map = HashMap::with_capacity(components.len());
        for (target, component_map) in components {
            let mut name_map = HashMap::with_capacity(component_map.len());
            for (package_canonical, supported) in component_map.keys() {
                // Add mappings for both the name in the manifest and the un-renamed package
                // if a rename mapping exists
                let unrenamed = inverse_renames.get(package_canonical.as_str());
                for package_alias in std::iter::once(package_canonical.as_str()).chain(unrenamed.into_iter().copied()) {
                    // If package is architecture dependent add it as $PACKAGE_NAME-$TRIPLE
                    if let SupportedTarget::Dependent(pkg_triple) = supported {
                        let full_name = format!("{}-{}", package_alias, pkg_triple);
                        name_map.insert(full_name, (package_canonical.to_string(), supported.clone()));
                    }
                    // If this package is for the current target or target-independent, add it
                    // without the suffix as well
                    if supported.supports(target) {
                        name_map.insert(
                            package_alias.to_string(),
                            (package_canonical.to_string(), supported.clone()),
                        );
                    }
                }
            }
            map.insert(target.clone(), name_map);
        }
        map
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

    pub fn get_profile_components(&self, profile: &str) -> Option<Vec<String>> {
        self.profiles.get(profile).cloned()
    }

    pub fn resolve_component_name_to_package(
        &self,
        target: &Triple,
        component: &str,
    ) -> Result<(String, SupportedTarget), Error> {
        let name_map = self
            .component_name_map
            .get(target)
            .ok_or_else(|| Error::UnknownTarget(target.to_string()))?;
        let package = name_map
            .get(component)
            .ok_or_else(|| Error::PackageUnknown(component.to_string(), SupportedTarget::Dependent(target.clone())))?;
        Ok(package.clone())
    }

    pub fn find_packages_for_install(
        &self,
        host: &Triple,
        spec: &InstallSpec,
    ) -> Result<HashSet<(String, SupportedTarget)>, Error> {
        let mut result = HashSet::new();
        let profile_components = self
            .profiles
            .get(&spec.profile)
            .ok_or_else(|| Error::UnknownProfile(spec.profile.to_string()))?;
        for component in profile_components {
            match self.resolve_component_name_to_package(host, component) {
                Ok(package) => {
                    result.insert(package);
                }
                Err(Error::PackageUnknown(..)) => {
                    // Since profiles can apparently contain components that
                    // aren't present on some platforms
                    // (e.g. rust-mingw), it is presumably
                    // safe to ignore this, given that the profile component
                    // list isn't supplied by the user.
                }
                Err(e) => return Err(e),
            }
        }
        for component in &spec.components {
            let package = self.resolve_component_name_to_package(host, component)?;
            result.insert(package);
        }
        for target in &spec.targets {
            let target = Triple::from_str(target)?;
            let component = format!("{}-{}", "rust-std", target);
            let package = self.resolve_component_name_to_package(host, &component)?;
            result.insert(package);
        }
        Ok(result)
    }

    pub fn find_downloads_for_install(&self, host: &Triple, spec: &InstallSpec) -> Result<Vec<ManifestPackage>, Error> {
        let mut result = Vec::new();
        let packages = self.find_packages_for_install(host, spec)?;
        for (package_name, target) in &packages {
            let builds = self
                .packages
                .get(package_name)
                .ok_or_else(|| Error::PackageUnknown(package_name.to_string(), target.clone()))?;
            let build = builds.get(target)?;
            let info = builds
                .info
                .as_ref()
                .ok_or_else(|| Error::MissingPackageVersion(package_name.clone()))?;
            let package = ManifestPackage {
                name: package_name.clone(),
                version: info.version.clone(),
                git_commit: info.git_commit.clone(),
                supported_target: target.clone(),
                tarballs: build.artifacts.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            };
            result.push(package)
        }
        Ok(result)
    }
}

impl TryFrom<&str> for Manifest {
    type Error = Error;

    fn try_from(string: &str) -> Result<Manifest, Error> {
        let parsed = manifest_v2::try_parse_manifest(string)?;
        Manifest::from_v2(parsed)
    }
}

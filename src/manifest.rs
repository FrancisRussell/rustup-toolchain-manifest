use crate::manifest_v2;
use crate::Error;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Manifest {
    version: String,
    date: NaiveDate,
    profiles: HashMap<String, Vec<String>>,
    renames: HashMap<String, String>,
    packages: HashMap<String, ()>,
}

impl Manifest {
    fn from_v2(parsed: manifest_v2::Manifest) -> Result<Manifest, Error> {
        let mut packages = HashMap::new();
        for (name, parsed_package) in parsed.packages {
            packages.insert(name, ());
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

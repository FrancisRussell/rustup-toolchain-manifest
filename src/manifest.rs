use crate::Error;
use chrono::NaiveDate;
use crate::manifest_v2;

#[derive(Debug)]
pub struct Manifest {
    version: String,
    date: NaiveDate,
}

impl Manifest {
    fn from_v2(parsed: manifest_v2::Manifest) -> Result<Manifest, Error> {
        let result = Manifest {
            version: parsed.manifest_version,
            date: parsed.date,
        };
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

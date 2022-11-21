use crate::Error;
use chrono::NaiveDate;
use toml::Value;

#[derive(Debug)]
pub struct Manifest {
    version: String,
    date: NaiveDate,
}

impl TryFrom<&Value> for Manifest {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Manifest, Error> {
        let table = if let Value::Table(table) = value {
            table
        } else {
            return Err(Error::IncorrectManifestStructure(
                "Top-level was not a table".into(),
            ));
        };
        let mut version = None;
        let mut date = None;
        for (key, value) in table {
            match key.as_str() {
                "date" => {
                    if let Value::String(value) = value {
                        date = Some(NaiveDate::parse_from_str(value, "%Y-%m-%d")?);
                    } else {
                        return Err(Error::IncorrectManifestStructure(
                            "Date not a string".into(),
                        ));
                    }
                }
                "manifest-version" => {
                    if let Value::String(value) = value {
                        version = Some(value.to_string());
                    } else {
                        return Err(Error::IncorrectManifestStructure(
                            "Version not a string".into(),
                        ));
                    }
                }
                "artifacts" => {}
                "pkg" => {}
                "profiles" => {}
                "renames" => {}
                _ => {
                    return Err(Error::IncorrectManifestStructure(format!(
                        "Unknown key: {}",
                        key
                    )))
                }
            }
        }
        let result = Manifest {
            version: version
                .ok_or_else(|| Error::IncorrectManifestStructure("Missing version".into()))?,
            date: date.ok_or_else(|| Error::IncorrectManifestStructure("Missing date".into()))?,
        };
        Ok(result)
    }
}

impl TryFrom<&str> for Manifest {
    type Error = Error;

    fn try_from(string: &str) -> Result<Manifest, Error> {
        let parsed = string.parse::<Value>()?;
        Manifest::try_from(&parsed)
    }
}

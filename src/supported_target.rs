use crate::error::Error;
use platforms::Platform;
use std::str::FromStr;

pub const TARGET_INDEPENDENT_NAME: &str = "*";

/// The targets a package supports
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SupportedTarget {
    /// The package supports all targets
    Independent,
    /// The package is specific to a particular target
    Dependent(Platform),
}

impl std::fmt::Display for SupportedTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SupportedTarget::Independent => TARGET_INDEPENDENT_NAME.fmt(formatter),
            SupportedTarget::Dependent(triple) => triple.fmt(formatter),
        }
    }
}

impl FromStr for SupportedTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<SupportedTarget, Self::Err> {
        Ok(if s == TARGET_INDEPENDENT_NAME {
            SupportedTarget::Independent
        } else {
            SupportedTarget::Dependent(
                Platform::find(s)
                    .ok_or_else(|| Error::UnknownTarget(s.to_string()))?
                    .clone(),
            )
        })
    }
}

impl SupportedTarget {
    #[must_use]
    pub fn supports(&self, other: &Platform) -> bool {
        match self {
            SupportedTarget::Independent => true,
            SupportedTarget::Dependent(triple) => triple == other,
        }
    }
}

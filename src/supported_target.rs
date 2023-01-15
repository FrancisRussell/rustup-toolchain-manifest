use std::str::FromStr;
use target_lexicon::Triple;

pub const TARGET_INDEPENDENT_NAME: &str = "*";

/// The targets a package supports
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SupportedTarget {
    /// The package supports all targets
    Independent,
    /// The package is specific to a particular target
    Dependent(Triple),
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
    type Err = target_lexicon::ParseError;

    fn from_str(s: &str) -> Result<SupportedTarget, Self::Err> {
        Ok(if s == TARGET_INDEPENDENT_NAME {
            SupportedTarget::Independent
        } else {
            SupportedTarget::Dependent(Triple::from_str(s)?)
        })
    }
}

impl SupportedTarget {
    #[must_use]
    pub fn supports(&self, other: &Triple) -> bool {
        match self {
            SupportedTarget::Independent => true,
            SupportedTarget::Dependent(triple) => triple == other,
        }
    }
}

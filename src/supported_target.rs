use std::str::FromStr;
use target_lexicon::Triple;

pub const TARGET_INDEPENDENT_NAME: &str = "*";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SupportedTarget {
    Independent,
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
    pub fn supports(&self, other: &Triple) -> bool {
        match self {
            SupportedTarget::Independent => true,
            SupportedTarget::Dependent(triple) => triple == other,
        }
    }
}

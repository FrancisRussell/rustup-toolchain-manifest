use chrono::NaiveDate;
use std::collections::VecDeque;
use std::str::FromStr;
use target_lexicon::Triple;
use thiserror::Error;

/// A Rust release channel
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Channel {
    /// The stable channel
    Stable,

    /// The beta channel
    Beta,

    /// The nightly channel
    Nightly,

    /// A specific Rust compiler version
    Version(u16, u16, Option<u16>),
}

impl std::fmt::Display for Channel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Channel::Stable => write!(formatter, "stable"),
            Channel::Beta => write!(formatter, "beta"),
            Channel::Nightly => write!(formatter, "nightly"),
            Channel::Version(major, minor, None) => {
                write!(formatter, "{}.{}", major, minor)
            }
            Channel::Version(major, minor, Some(patch)) => {
                write!(formatter, "{}.{}.{}", major, minor, patch)
            }
        }
    }
}

/// Errors which can occur while parsing a channel name
#[derive(Clone, Debug, Error)]
pub enum ChannelParseError {
    /// An integer component failed to parse
    #[error("Could not parse version number component as integer")]
    IntegerParse,

    /// A version had an invalid number of components
    #[error("Incorrect number of components in version: {0}")]
    InvalidVersionComponentCount(usize),
}

impl FromStr for Channel {
    type Err = ChannelParseError;

    fn from_str(string: &str) -> Result<Channel, ChannelParseError> {
        match string {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            _ => {
                let components: Result<Vec<u16>, _> = string.split('.').map(u16::from_str).collect();
                let components = components.map_err(|_| ChannelParseError::IntegerParse)?;
                if components.len() < 2 || components.len() > 3 {
                    return Err(ChannelParseError::InvalidVersionComponentCount(components.len()));
                }
                assert!(components.len() >= 2);
                assert!(components.len() <= 3);
                Ok(Channel::Version(
                    components[0],
                    components[1],
                    components.get(2).copied(),
                ))
            }
        }
    }
}

/// Specification of a Rust toolchain (as typically given to Rustup)
#[derive(Debug, Clone)]
pub struct Toolchain {
    /// The release channel
    pub channel: Channel,

    /// The manifest date
    pub date: Option<NaiveDate>,

    /// The target triple
    pub host: Option<Triple>,
}

impl Toolchain {
    /// Returns the manifest URL for the specified toolchain
    #[must_use]
    pub fn manifest_url(&self) -> String {
        if let Some(date) = &self.date {
            format!(
                "https://static.rust-lang.org/dist/{}/channel-rust-{}.toml",
                date, self.channel
            )
        } else {
            format!("https://static.rust-lang.org/dist/channel-rust-{}.toml", self.channel)
        }
    }
}

/// Parse errors for a toolchain string
#[derive(Debug, Clone, Error)]
pub enum ParseError {
    /// The channel name was invalid
    #[error("Failed to parse channel: {0}")]
    Channel(#[from] ChannelParseError),

    /// The target name was invalid
    #[error("Failed to target: {0}")]
    Target(#[from] target_lexicon::ParseError),
}

fn intersperse_hyphen<I: Iterator<Item = S>, S: AsRef<str>>(iter: I) -> String {
    // The following code could be written much more nicely with intersperse
    // https://github.com/rust-lang/rust/issues/79524
    let mut result = String::new();
    let mut first = true;
    for component in iter {
        if first {
            first = false;
        } else {
            result.push('-');
        }
        result.push_str(component.as_ref());
    }
    result
}

impl FromStr for Toolchain {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Toolchain, ParseError> {
        let mut split: VecDeque<_> = string.split('-').collect();
        let channel = Channel::from_str(split.pop_front().unwrap_or_default())?;
        let mut result = Toolchain {
            channel,
            date: None,
            host: None,
        };
        if split.len() >= 3 {
            let date_candidate = intersperse_hyphen(split.range(0..3));
            if let Ok(date) = NaiveDate::from_str(&date_candidate) {
                result.date = Some(date);
                split.drain(0..3);
            }
        }
        if !split.is_empty() {
            let host_candidate = intersperse_hyphen(split.iter());
            let host = Triple::from_str(&host_candidate)?;
            result.host = Some(host);
        }
        Ok(result)
    }
}

impl std::fmt::Display for Toolchain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.channel.fmt(formatter)?;
        if let Some(date) = &self.date {
            write!(formatter, "-{}", date)?;
        }
        if let Some(host) = &self.host {
            write!(formatter, "-{}", host)?;
        }
        Ok(())
    }
}

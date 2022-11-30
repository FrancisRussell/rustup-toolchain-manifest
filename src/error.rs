use crate::supported_target::SupportedTarget;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Manifest had incorect structure: {0}")]
    IncorrectManifestStructure(String),

    #[error("Package {0} listed as both target-dependent and independent")]
    ConflictingTargetDependence(String),

    #[error("Package \"rust\" was missing from manifest")]
    RustMissing,

    #[error("Package {0} unknown for target {1}")]
    PackageUnknown(String, SupportedTarget),

    #[error("Unknown target: {0}")]
    UnknownTarget(String),

    #[error("Unknown profile: {0}")]
    UnknownProfile(String),

    #[error("Failed to parse target triple: {0}")]
    TargetParse(#[from] target_lexicon::ParseError),

    #[error("Attempted to treat package {0} as architecture independent")]
    PackageNotTargetIndependent(String),

    #[error("Package {0} unavailable for target {1}")]
    PackageUnavailable(String, SupportedTarget),
}

use crate::supported_target::SupportedTarget;
use thiserror::Error;

/// Errors that can occur during manifest parse or querying
#[derive(Debug, Error)]
pub enum Error {
    /// The manifest TOML failed to deserialize correctly
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    /// The manifest did not conform to the expected structure (parsing was fine
    /// though)
    #[error("Manifest had incorect structure: {0}")]
    IncorrectManifestStructure(String),

    /// A package was listed as both architecture-specific but also independent
    #[error("Package {0} listed as both target-dependent and independent")]
    ConflictingTargetDependence(String),

    /// The `rust` package was missing from the manifest
    #[error("Package \"rust\" was missing from manifest")]
    RustMissing,

    /// The specified package was unknown the specified target
    #[error("Package {0} unknown for target {1}")]
    PackageUnknown(String, SupportedTarget),

    /// The specified target architecture was unknown
    #[error("Unknown target: {0}")]
    UnknownTarget(String),

    /// The specified toolchain profile was unknown
    #[error("Unknown profile: {0}")]
    UnknownProfile(String),

    /// A target triple could not be parsed
    #[error("Failed to parse target triple: {0}")]
    TargetParse(#[from] target_lexicon::ParseError),

    /// A target-dependent package was referred to in an target-independent
    /// context.
    #[error("Attempted to treat package {0} as architecture independent")]
    PackageNotTargetIndependent(String),

    /// The package was not available for the specified target (but was
    /// recognized)
    #[error("Package {0} unavailable for target {1}")]
    PackageUnavailable(String, SupportedTarget),

    /// The package's version information was missing
    #[error("Missing versioning information for package {0}")]
    MissingPackageVersion(String),
}

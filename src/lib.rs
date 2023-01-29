#![warn(clippy::pedantic)]
#![allow(
    clippy::uninlined_format_args,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
#![forbid(unsafe_code)]

mod error;

/// Types related to digest values
pub mod hash_value;

/// Types related to toolchain manifests
pub mod manifest;

mod manifest_v2;
mod supported_target;

/// Types related to toolchain specification
pub mod toolchain;

pub use error::Error;
pub use hash_value::HashValue;
pub use manifest::{InstallSpec, Manifest};
pub use supported_target::SupportedTarget;
pub use toolchain::Toolchain;

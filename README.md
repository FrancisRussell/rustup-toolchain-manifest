# Rust Toolchain Manifest

This is a library which is capable of parsing the v2 Rust toolchain manifest and
doing some basic queries on it. It is in no way offical, and the Rust
toolchain manifest has the ability to change arbitrarily at any time. It was
written simply because I could not find an existing library that did this.

The Rust toolchain manifest format has clearly been extended over time and some
aspects of it, in particular how missing packages and renames are handled, is
quite confusing.  This library is a best-effort attempt to reverse-engineer the
underlying meaning.

This package is named `rustup-toolchain-manifest` only because
`rust-toolchain-manifest` seems to have been name squatted.

## Example usage (from `examples/readme.rs`)

```rust
fn main() {
    let toolchain = Toolchain::from_str("nightly-x86_64-unknown-linux-gnu")
        .expect("Failed to parse toolchain");
    println!("Toolchain specification:\n{:#?}\n", toolchain);

    let manifest_url = toolchain.manifest_url();
    println!("Downloading manifest from: {}", manifest_url);
    let manifest = reqwest::blocking::get(&manifest_url)
        .expect("Failed to fetch manifest")
        .text()
        .expect("Could not get response text");
    println!(
        "Successfully retrieved manifest of {} bytes.",
        manifest.len()
    );

    let manifest =
        Manifest::try_from(manifest.as_str()).expect("Failed to parse manifest");
    let install_spec = InstallSpec {
        profile: "default".into(),
        components: HashSet::new(),
        targets: ["wasm32-unknown-unknown"]
            .into_iter()
            .map(String::from)
            .collect(),
    };

    let target = toolchain
        .host
        .expect("Host missing for previously specified toolchain");
    println!(
        "Finding packages on {} for install specification:\n{:#?}\n",
        target, install_spec
    );
    let packages = manifest
        .find_packages_for_install(&target, &install_spec)
        .expect("Failed to find packages");
    println!("The following packages are required:");
    for (name, target) in packages {
        println!("{} ({})", name, target);
    }
}
```

Output:
```
Toolchain specification:
Toolchain {
    channel: Nightly,
    date: None,
    host: Some(
        Triple {
            architecture: X86_64,
            vendor: Unknown,
            operating_system: Linux,
            environment: Gnu,
            binary_format: Elf,
        },
    ),
}

Downloading manifest from: https://static.rust-lang.org/dist/channel-rust-nightly.toml
Successfully retrieved manifest of 769388 bytes.
Finding packages on x86_64-unknown-linux-gnu for install specification:
InstallSpec {
    profile: "default",
    components: {},
    targets: {
        "wasm32-unknown-unknown",
    },
}

The following packages are required:
rustc (x86_64-unknown-linux-gnu)
clippy-preview (x86_64-unknown-linux-gnu)
rust-std (wasm32-unknown-unknown)
rust-std (x86_64-unknown-linux-gnu)
rustfmt-preview (x86_64-unknown-linux-gnu)
rust-docs (x86_64-unknown-linux-gnu)
cargo (x86_64-unknown-linux-gnu)
```

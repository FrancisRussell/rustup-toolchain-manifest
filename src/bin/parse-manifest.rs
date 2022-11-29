use clap::Parser;
use rust_toolchain_manifest::{InstallSpec, Manifest};
use std::path::PathBuf;
use std::str::FromStr;
use target_lexicon::Triple;

#[derive(Debug, Parser)]
#[clap(author, version)]
struct Cli {
    /// Input file
    input_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let input_file = cli.input_file;
    let content = std::fs::read_to_string(&input_file).expect("Failed to read input file");
    let manifest = Manifest::try_from(content.as_str()).expect("Failed to parse manifest");
    let spec = InstallSpec {
        profile: "minimal".into(),
        components: ["clippy", "rust-src"]
            .into_iter()
            .map(String::from)
            .collect(),
        targets: ["wasm32-unknown-unknown"]
            .into_iter()
            .map(String::from)
            .collect(),
    };
    let host = Triple::from_str("x86_64-unknown-linux-gnu").expect("Failed to parse triple");
    println!("For target {}, finding the following toolchain:\n{:#?}", host, spec);
    println!();
    let packages = manifest
        .find_needed_packages(&host, &spec)
        .expect("Failed to resolve install specification");
    println!("Packages:\n{:#?}", packages);
}

use clap::Parser;
use rust_toolchain_manifest::Manifest;
use std::path::PathBuf;

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
    println!("{:#?}", manifest);
}

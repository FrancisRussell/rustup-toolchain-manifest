use rustup_toolchain_manifest::{Manifest, Toolchain, InstallSpec};
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let toolchain = Toolchain::from_str("nightly-x86_64-unknown-linux-gnu").expect("Failed to parse toolchain");
    println!("Toolchain specification:\n{:#?}\n", toolchain);

    let manifest_url = toolchain.manifest_url();
    println!("Downloading manifest from: {}", manifest_url);
    let manifest = reqwest::blocking::get(&manifest_url)
        .expect("Failed to fetch manifest")
        .text()
        .expect("Could not get response text");
    println!("Successfully retrieved manifest of {} bytes.", manifest.len());

    let manifest = Manifest::try_from(manifest.as_str()).expect("Failed to parse manifest");
    let install_spec = InstallSpec {
        profile: "default".into(),
        components: HashSet::new(),
        targets: ["wasm32-unknown-unknown"].into_iter().map(String::from).collect(),
    };

    let target = toolchain.host.expect("Host missing for previously specified toolchain");
    println!(
        "Finding packages on {} for install specification:\n{:#?}\n",
        target,
        install_spec
    );
    let packages = manifest.find_packages_for_install(&target, &install_spec).expect("Failed to find packages");
    println!("The following packages are required:");
    for (name, target) in packages {
        println!("{} ({})", name, target);
    }
}

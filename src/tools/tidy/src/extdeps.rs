//! Check for external package sources. Allow only vendorable packages.

use std::fs;
use std::path::Path;

/// List of allowed sources for packages.
const ALLOWED_SOURCES: &[&str] = &["\"registry+https://github.com/rust-lang/crates.io-index\"",
"\"git+https://github.com/anza-xyz/compiler-builtins?tag=solana-tools-v1.44#5380e204aab5be48c798679cc52b1f871c8ad4bc\"",
"\"git+https://github.com/anza-xyz/rustc-build-sysroot?tag=solana-tools-v1.44#4f19dfe109fc80efbe62b6de9781125bc7ce7485\""];

/// Checks for external package sources. `root` is the path to the directory that contains the
/// workspace `Cargo.toml`.
pub fn check(root: &Path, bad: &mut bool) {
    for &(workspace, _, _) in crate::deps::WORKSPACES {
        // FIXME check other workspaces too
        // `Cargo.lock` of rust.
        let path = root.join(workspace).join("Cargo.lock");

        if !path.exists() {
            tidy_error!(bad, "the `{workspace}` workspace doesn't have a Cargo.lock");
            continue;
        }

        // Open and read the whole file.
        let cargo_lock = t!(fs::read_to_string(&path));

        // Process each line.
        for line in cargo_lock.lines() {
            // Consider only source entries.
            if !line.starts_with("source = ") {
                continue;
            }

            // Extract source value.
            let source = line.split_once('=').unwrap().1.trim();

            // Ensure source is allowed.
            if !ALLOWED_SOURCES.contains(&&*source) {
                tidy_error!(bad, "invalid source: {}", source);
            }
        }
    }
}

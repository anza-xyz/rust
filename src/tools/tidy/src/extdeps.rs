//! Check for external package sources. Allow only vendorable packages.

use std::fs;
use std::path::Path;

/// List of allowed sources for packages.
const ALLOWED_SOURCES: &[&str] = &["\"registry+https://github.com/rust-lang/crates.io-index\"",
"\"git+https://github.com/anza-xyz/compiler-builtins?tag=solana-tools-v1.43#8e7bd0b5b32d58c96aca7c6d45d3989211e1a378\"",
"\"git+https://github.com/anza-xyz/rustc-build-sysroot?tag=solana-tools-v1.43#54b64222f01199bfc1fc7ac36f244cef4f573c7e\""];

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

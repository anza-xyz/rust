//! Check for external package sources. Allow only vendorable packages.

use std::fs;
use std::path::Path;

use crate::deps::WorkspaceInfo;
use crate::diagnostics::TidyCtx;

/// List of allowed sources for packages.
const ALLOWED_SOURCES: &[&str] = &[
    r#""registry+https://github.com/rust-lang/crates.io-index""#,
    // This is `rust_team_data` used by `site` in src/tools/rustc-perf,
    r#""git+https://github.com/rust-lang/team#a5260e76d3aa894c64c56e6ddc8545b9a98043ec""#,
    r#""git+https://github.com/anza-xyz/compiler-builtins?tag=solana-tools-v1.44#4abd7f45fc621344e4b9d1df37a45b7445f03c71""#,
    r#""git+https://github.com/anza-xyz/rustc-build-sysroot?tag=solana-tools-v1.44#4f19dfe109fc80efbe62b6de9781125bc7ce7485""#,
];

/// Checks for external package sources. `root` is the path to the directory that contains the
/// workspace `Cargo.toml`.
pub fn check(root: &Path, tidy_ctx: TidyCtx) {
    let mut check = tidy_ctx.start_check("extdeps");

    for &WorkspaceInfo { path, submodules, .. } in crate::deps::WORKSPACES {
        if crate::deps::has_missing_submodule(root, submodules) {
            continue;
        }

        // FIXME check other workspaces too
        // `Cargo.lock` of rust.
        let lockfile = root.join(path).join("Cargo.lock");

        if !lockfile.exists() {
            check.error(format!("the `{path}` workspace doesn't have a Cargo.lock"));
            continue;
        }

        // Open and read the whole file.
        let cargo_lock = t!(fs::read_to_string(&lockfile));

        // Process each line.
        for line in cargo_lock.lines() {
            // Consider only source entries.
            if !line.starts_with("source = ") {
                continue;
            }

            // Extract source value.
            let source = line.split_once('=').unwrap().1.trim();

            // Ensure source is allowed.
            if !ALLOWED_SOURCES.contains(&source) {
                check.error(format!("invalid source: {}", source));
            }
        }
    }
}

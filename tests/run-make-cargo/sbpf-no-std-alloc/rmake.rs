// Builds a `#![no_std]` program that uses `alloc` for the sbpf targets, and runs it in the VM
// when a runner is configured.
//
// The test binaries produced by `x.py test library/{core,alloc}` always link `std`. `std`
// provides its own `#[alloc_error_handler]`, so `alloc`'s default `__rdl_alloc_error_handler`
// is never referenced by them. Only programs that link `core` and `alloc` but not `std` need
// that symbol, and so only those notice when `alloc` stops providing it. Linking such a program
// is what this test checks. `-Zbuild-std` is used so that the in-tree `core` and `alloc` are
// built for each target, regardless of which targets the sysroot under test was built for.
//
// If cargo's `CARGO_TARGET_<TRIPLE>_RUNNER` is set for the target (e.g. to
// `cargo-run-solana-tests --heap-size <size>`), the program is run with it too. It succeeds if
// its `entrypoint` returns 0. The heap size must be at least that used by the program (1 MiB).

//@ needs-llvm-components: sbf
//@ needs-rust-lld

#![deny(warnings)]

use run_make_support::{cargo, cmd, path};

const TARGETS: &[&str] = &[
    "sbf-solana-solana",
    "sbpf-solana-solana",
    "sbpfv1-solana-solana",
    "sbpfv2-solana-solana",
    "sbpfv3-solana-solana",
];

fn main() {
    for target in TARGETS {
        build_and_run(target, "");
    }
    // The ALU32 variant of SBPFv3 is tested separately.
    build_and_run("sbpfv3-solana-solana", "-Ctarget-feature=+alu32");
}

fn build_and_run(target: &str, rustflags: &str) {
    let suffix = if rustflags.is_empty() { "" } else { "-flags" };
    let target_dir = path(format!("target-{target}{suffix}"));
    cargo()
        .current_dir("program")
        .args(&[
            "build",
            "--release",
            "--manifest-path",
            "Cargo.toml",
            "-Zbuild-std=core,alloc",
            "-Zbuild-std-features=compiler-builtins-mem",
            "--target",
            target,
        ])
        .env("RUSTFLAGS", rustflags)
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .context(&format!("building for `{target}` with RUSTFLAGS=`{rustflags}` failed"))
        .run();

    let runner_var = format!("CARGO_TARGET_{}_RUNNER", target.to_uppercase().replace('-', "_"));
    let Ok(runner) = std::env::var(&runner_var) else {
        return;
    };
    let program = target_dir.join(target).join("release").join("sbpf_no_std_alloc.so");
    assert!(program.exists(), "{} was not built", program.display());

    let runner = runner.split_whitespace().collect::<Vec<_>>();
    let (runner_program, runner_args) = runner.split_first().expect("runner must not be empty");
    cmd(runner_program)
        .args(runner_args)
        .arg(&program)
        .context(&format!("running the program for `{target}` with RUSTFLAGS=`{rustflags}` failed"))
        .run();
}

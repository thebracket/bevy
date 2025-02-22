use xshell::{cmd, pushd};

use bitflags::bitflags;

bitflags! {
    struct Check: u32 {
        const FORMAT = 0b00000001;
        const CLIPPY = 0b00000010;
        const COMPILE_FAIL = 0b00000100;
        const TEST = 0b00001000;
        const DOC_TEST = 0b00010000;
        const DOC_CHECK = 0b00100000;
        const BENCH_CHECK = 0b01000000;
        const EXAMPLE_CHECK = 0b10000000;
        const COMPILE_CHECK = 0b100000000;
    }
}

fn main() {
    // When run locally, results may differ from actual CI runs triggered by
    // .github/workflows/ci.yml
    // - Official CI runs latest stable
    // - Local runs use whatever the default Rust is locally

    let what_to_run = match std::env::args().nth(1).as_deref() {
        Some("format") => Check::FORMAT,
        Some("clippy") => Check::CLIPPY,
        Some("compile-fail") => Check::COMPILE_FAIL,
        Some("test") => Check::TEST,
        Some("doc-test") => Check::DOC_TEST,
        Some("doc-check") => Check::DOC_CHECK,
        Some("bench-check") => Check::BENCH_CHECK,
        Some("example-check") => Check::EXAMPLE_CHECK,
        Some("lints") => Check::FORMAT | Check::CLIPPY,
        Some("doc") => Check::DOC_TEST | Check::DOC_CHECK,
        Some("compile") => {
            Check::COMPILE_FAIL | Check::BENCH_CHECK | Check::EXAMPLE_CHECK | Check::COMPILE_CHECK
        }
        _ => Check::all(),
    };

    if what_to_run.contains(Check::FORMAT) {
        // See if any code needs to be formatted
        cmd!("cargo fmt --all -- --check")
            .run()
            .expect("Please run 'cargo fmt --all' to format your code.");
    }

    if what_to_run.contains(Check::CLIPPY) {
        // See if clippy has any complaints.
        // - Type complexity must be ignored because we use huge templates for queries
        cmd!("cargo clippy --workspace --all-targets --all-features -- -A clippy::type_complexity -W clippy::doc_markdown -D warnings")
        .run()
        .expect("Please fix clippy errors in output above.");
    }

    if what_to_run.contains(Check::COMPILE_FAIL) {
        // Run UI tests (they do not get executed with the workspace tests)
        // - See crates/bevy_ecs_compile_fail_tests/README.md
        let _bevy_ecs_compile_fail_tests = pushd("crates/bevy_ecs_compile_fail_tests")
            .expect("Failed to navigate to the 'bevy_ecs_compile_fail_tests' crate");
        cmd!("cargo test --target-dir ../../target")
            .run()
            .expect("Compiler errors of the ECS compile fail tests seem to be different than expected! Check locally and compare rust versions.");
    }

    if what_to_run.contains(Check::TEST) {
        // Run tests (except doc tests and without building examples)
        cmd!("cargo test --workspace --lib --bins --tests --benches")
            .run()
            .expect("Please fix failing tests in output above.");
    }

    if what_to_run.contains(Check::DOC_TEST) {
        // Run doc tests
        cmd!("cargo test --workspace --doc")
            .run()
            .expect("Please fix failing doc-tests in output above.");
    }

    if what_to_run.contains(Check::DOC_CHECK) {
        // Check that building docs work and does not emit warnings
        std::env::set_var("RUSTDOCFLAGS", "-D warnings");
        cmd!("cargo doc --workspace --all-features --no-deps --document-private-items")
            .run()
            .expect("Please fix doc warnings in output above.");
    }

    if what_to_run.contains(Check::COMPILE_FAIL) {
        // Check that benches are building
        let _benches = pushd("benches").expect("Failed to navigate to the 'benches' folder");
        cmd!("cargo check --benches --target-dir ../target")
            .run()
            .expect("Failed to check the benches.");
    }

    if what_to_run.contains(Check::EXAMPLE_CHECK) {
        // Build examples and check they compile
        cmd!("cargo check --workspace --examples")
            .run()
            .expect("Please fix failing doc-tests in output above.");
    }

    if what_to_run.contains(Check::COMPILE_CHECK) {
        // Build examples and check they compile
        cmd!("cargo check --workspace")
            .run()
            .expect("Please fix failing doc-tests in output above.");
    }
}

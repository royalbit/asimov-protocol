//! End-to-end test modules for asimov CLI
//!
//! Split into modules to keep each under 1500 lines.

use std::path::PathBuf;

/// Get the path to the asimov binary
pub fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    path.push("asimov");

    if !path.exists() {
        // Fall back to debug build
        path.pop();
        path.pop();
        path.push("debug");
        path.push("asimov");
    }

    path
}

mod help;
mod init;
mod lint;
mod misc;
mod validate;

// src/bin/uitest.rs
use std::{
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> io::Result<()> {
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()?;

    if !status.success() {
        std::process::exit(1);
    }

    let exe = PathBuf::from("target/release/dinglebob");
    if !exe.exists() {
        std::process::exit(1);
    }

    let test_dir = Path::new("uitest");
    if !test_dir.exists() {
        std::process::exit(1);
    }

    let mut tests: Vec<PathBuf> = fs::read_dir(test_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension() == Some(OsStr::new("dingle")))
        .collect();

    tests.sort();

    let mut any_failed = false;

    for test_path in tests {
        let output = Command::new(&exe)
            // Try to disable colors at the source (best-effort)
            .env("NO_COLOR", "1")
            .env("CLICOLOR", "0")
            .env("CLICOLOR_FORCE", "0")
            .env("TERM", "dumb")
            .arg(&test_path)
            .output()?;

        let stderr_path = PathBuf::from(format!("{}.stderr", test_path.display()));

        let stdout_clean = strip_ansi_escapes::strip(&output.stdout);
        let stderr_clean = strip_ansi_escapes::strip(&output.stderr);

        let stderr_path = PathBuf::from(format!("{}.stderr", test_path.display()));

        let stdout_clean = strip_ansi_escapes::strip(&output.stdout);
        let stderr_clean = strip_ansi_escapes::strip(&output.stderr);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&stdout_clean);
        bytes.extend_from_slice(&stderr_clean);

        fs::write(&stderr_path, bytes)?;

        if !output.status.success() {
            any_failed = true;
        }
    }

    if any_failed {
        std::process::exit(1);
    }

    Ok(())
}

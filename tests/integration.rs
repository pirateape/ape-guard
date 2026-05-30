// ApeGuard Integration Tests
use assert_cmd::Command;
use predicates::prelude::*;

const BINARY: &str = "apeguard";

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ApeGuard v0.1.0"))
        .stdout(predicate::str::contains("EL-2.0"));
}

#[test]
fn test_init_creates_config() {
    let tmpdir = tempfile::tempdir().unwrap();
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg("init")
        .arg(tmpdir.path())
        .assert()
        .success();

    // Check config file was created
    let config_path = tmpdir.path().join(".apeguard.yaml");
    assert!(config_path.exists(), "Config file should exist after init");

    // Verify content
    let content = std::fs::read_to_string(config_path).unwrap();
    assert!(content.contains("ApeGuard"));
}

#[test]
fn test_init_fails_if_exists() {
    let tmpdir = tempfile::tempdir().unwrap();
    let config_path = tmpdir.path().join(".apeguard.yaml");
    std::fs::write(&config_path, "existing: true").unwrap();

    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg("init")
        .arg(tmpdir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_completions_bash() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["completions", "zsh"])
        .assert()
        .success();
}

#[test]
fn test_config_validate() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["config", "validate"])
        .assert()
        .success();
}

#[test]
fn test_config_paths() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["config", "paths"])
        .assert()
        .success()
        .stdout(predicate::str::contains(".apeguard.yaml"));
}

#[test]
fn test_scan_on_empty_dir() {
    let tmpdir = tempfile::tempdir().unwrap();
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["scan", tmpdir.path().to_str().unwrap(), "--layers", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ApeGuard Scan Complete"));
}

#[test]
fn test_scan_with_config() {
    let tmpdir = tempfile::tempdir().unwrap();

    // Create a config file
    let config = r#"
layers:
  - 1
  - 2
severity: "high"
"#;
    std::fs::write(tmpdir.path().join(".apeguard.yaml"), config).unwrap();

    // Init should fail since config exists
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["init", tmpdir.path().to_str().unwrap()])
        .assert()
        .failure();
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Security posture assessment"))
        .stdout(predicate::str::contains("scan"))
        .stdout(predicate::str::contains("report"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("version"));
}

#[test]
fn test_scan_help() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["scan", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--layers"))
        .stdout(predicate::str::contains("--severity"))
        .stdout(predicate::str::contains("--format"));
}

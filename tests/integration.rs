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
        .stdout(predicate::str::contains("ApeGuard v"))
        .stdout(predicate::str::contains("Elastic-2.0"));
}

#[test]
fn test_init_creates_config() {
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg("init").arg(tmpdir.path()).assert().success();

    // Check config file was created
    let config_path = tmpdir.path().join(".apeguard.yaml");
    assert!(config_path.exists(), "Config file should exist after init");

    // Verify content
    let content = std::fs::read_to_string(config_path).unwrap();
    assert!(content.contains("ApeGuard"));
}

#[test]
fn test_init_fails_if_exists() {
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");
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
    cmd.args(["completions", "zsh"]).assert().success();
}

#[test]
fn test_config_validate() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["config", "validate"]).assert().success();
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
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args(["scan", tmpdir.path().to_str().unwrap(), "--layers", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ApeGuard Scan Complete"));
}

#[test]
fn test_scan_with_config() {
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");

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

#[test]
fn test_full_scan_with_findings_and_formats() {
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");

    // Create test files with known vulnerabilities
    // Gitleaks should detect the API key pattern
    std::fs::write(
        tmpdir.path().join("config.py"),
        r#"
# Test configuration file with secrets
API_KEY = "sk-aaaaaabbbbbbccccccccccdddddddd"
password = "hunter2"
db_url = "postgresql://admin:secret123@localhost:5432/mydb"
"#,
    )
    .unwrap();

    // Semgrep should detect eval() usage
    std::fs::write(
        tmpdir.path().join("app.py"),
        r#"
import os

def process_input(user_input):
    # Insecure eval usage
    result = eval(user_input)
    return result

def run_command(cmd):
    # Command injection
    os.system(cmd)
    return True
"#,
    )
    .unwrap();

    // Create a simple file to ensure enough content
    std::fs::write(
        tmpdir.path().join("README.md"),
        "# Test Project\nJust a test.",
    )
    .unwrap();

    // Run scan with all formats
    let output_dir = tmpdir.path().join("reports");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args([
        "scan",
        tmpdir.path().to_str().unwrap(),
        "--layers",
        "1,2", // Gitleaks + Semgrep (skip Trivy for speed)
        "--format",
        "md,json,sarif",
        "--output-dir",
        output_dir.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("ApeGuard Scan Complete"));

    // Verify markdown reports were generated
    assert!(
        output_dir.join("technical-report.md").exists(),
        "Technical report should exist"
    );
    assert!(
        output_dir.join("executive-report.md").exists(),
        "Executive report should exist"
    );
    assert!(
        output_dir.join("roadmap-report.md").exists(),
        "Roadmap report should exist"
    );

    // Verify JSON report was generated
    let json_path = output_dir.join("apeguard-report.json");
    assert!(json_path.exists(), "JSON report should exist");
    let json_content = std::fs::read_to_string(&json_path).unwrap();
    let json_parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("JSON report should be valid JSON");
    assert!(
        json_parsed["findings"].is_array(),
        "JSON should contain findings array"
    );
    assert!(
        json_parsed["scorecard"].is_object(),
        "JSON should contain scorecard"
    );

    // Verify SARIF report was generated
    let sarif_path = output_dir.join("apeguard-report.sarif");
    assert!(sarif_path.exists(), "SARIF report should exist");
    let sarif_content = std::fs::read_to_string(&sarif_path).unwrap();
    let sarif_parsed: serde_json::Value =
        serde_json::from_str(&sarif_content).expect("SARIF report should be valid JSON");
    assert_eq!(sarif_parsed["version"], "2.1.0");
    assert!(
        sarif_parsed["runs"].is_array(),
        "SARIF should contain runs array"
    );
    assert!(
        sarif_parsed["runs"][0]["results"].is_array(),
        "SARIF should contain results array"
    );

    // Now test report regeneration from cache
    let report_dir = tmpdir.path().join("regen-reports");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args([
        "report",
        tmpdir.path().to_str().unwrap(),
        "--output-dir",
        report_dir.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("ApeGuard Report Regeneration"));

    // Verify regenerated reports
    assert!(
        report_dir.join("technical-report.md").exists(),
        "Regenerated technical report should exist"
    );
}

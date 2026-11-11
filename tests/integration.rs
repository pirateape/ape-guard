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

#[test]
fn test_resume_reloads_findings_from_previous_run() {
    // Regression test for the resume bug: `--resume` must reload findings
    // persisted by the previous run. With all layers already complete, a
    // resumed scan must regenerate the SAME findings — not an empty report.
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");

    // Seed a leaky token so layer 1 (Gitleaks) reliably finds it. Scanners not
    // installed on the machine report "not installed" and are still recorded
    // in the JSONL, so resume works regardless of which scanners are present.
    std::fs::write(
        tmpdir.path().join("secrets.env"),
        "token = \"ghp_R8qYv2XwB7nK4mT9zJcL3dF6aS1hG0qW5e\"\n",
    )
    .unwrap();

    let target = tmpdir.path().to_str().unwrap().to_string();
    let output_dir = tmpdir.path().join("reports");
    let output_dir_str = output_dir.to_str().unwrap().to_string();

    // Run 1: full scan (no cache so the run is deterministic)
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args([
        "scan",
        &target,
        "--layers",
        "1",
        "--no-cache",
        "--format",
        "md,json",
        "--output-dir",
        &output_dir_str,
    ])
    .assert()
    .success();

    // The JSONL must contain finding lines (persisted for resume)
    let jsonl_path = output_dir.join("found_findings.jsonl");
    assert!(
        jsonl_path.exists(),
        "found_findings.jsonl should exist after a scan"
    );
    let jsonl = std::fs::read_to_string(&jsonl_path).unwrap();
    assert!(
        jsonl.contains("\"status\":\"complete\"") || jsonl.contains("\"status\": \"complete\""),
        "JSONL should contain a complete status line"
    );

    // Count finding lines in run 1
    let finding_lines_run1 = jsonl
        .lines()
        .filter(|l| l.contains("\"status\":\"finding\"") || l.contains("\"status\": \"finding\""))
        .count();
    assert!(
        finding_lines_run1 > 0,
        "JSONL should persist findings — gitleaks must detect the seeded token"
    );

    // Run 2: resume with all layers already complete. The report must NOT be
    // empty when run 1 produced findings — resume reloads persisted results.
    if finding_lines_run1 > 0 {
        let json_report = output_dir.join("apeguard-report.json");
        let before: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&json_report).expect("run-1 JSON report"),
        )
        .expect("run-1 JSON should parse");

        let mut cmd = Command::cargo_bin(BINARY).unwrap();
        cmd.args([
            "scan",
            &target,
            "--layers",
            "1",
            "--resume",
            "--no-cache",
            "--format",
            "md,json",
            "--output-dir",
            &output_dir_str,
        ])
        .assert()
        .success();

        let after: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&json_report).expect("resumed JSON report"),
        )
        .expect("resumed JSON should parse");

        assert_eq!(
            before["findings"].as_array().map(|a| a.len()),
            after["findings"].as_array().map(|a| a.len()),
            "resumed scan must reload the same findings as the original run \
             (empty reports on full resume were the pre-fix bug)"
        );
    }
}

#[test]
fn test_scan_id_consistent_between_report_and_cache() {
    // Regression test for the dual-scan-ID bug: the scan ID printed in reports
    // must be the same ID stored in the cache, so `report --snapshot <id>` and
    // `compare <id>` work with the ID the user can actually see.
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir for integration test");
    std::fs::write(
        tmpdir.path().join("config.py"),
        "API_KEY = \"sk-aaaaaabbbbbbccccccccccdddddddd\"",
    )
    .unwrap();

    let target = tmpdir.path().to_str().unwrap().to_string();
    let output_dir = tmpdir.path().join("reports");
    let output_dir_str = output_dir.to_str().unwrap().to_string();

    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args([
        "scan",
        &target,
        "--layers",
        "1",
        "--format",
        "md,json",
        "--output-dir",
        &output_dir_str,
    ])
    .assert()
    .success();

    // The JSON report carries the canonical scan ID
    let json_report = output_dir.join("apeguard-report.json");
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json_report).expect("JSON report"))
            .expect("JSON should parse");
    let scan_id = report["summary"]["scan_id"]
        .as_str()
        .expect("scan_id in JSON report")
        .to_string();

    assert!(
        scan_id.starts_with("scan_"),
        "scan ID should use the canonical scan_{{timestamp}} format, got: {scan_id}"
    );

    // That same ID must be usable as a snapshot for report regeneration —
    // before the fix this failed with "Scan snapshot ... not found in cache".
    let regen_dir = tmpdir.path().join("regen-by-id");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.args([
        "report",
        &target,
        "--snapshot",
        &scan_id,
        "--output-dir",
        regen_dir.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(&scan_id));

    assert!(
        regen_dir.join("technical-report.md").exists(),
        "report --snapshot <id from report> should regenerate reports"
    );
}

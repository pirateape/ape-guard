// ApeGuard False Positive Suppression Pipeline
// Multi-stage filter applied after dedup and before LLM to eliminate findings
// that are almost certainly false positives, saving LLM API calls and reducing noise.
//
// Filter stages:
//   1. Path-based exclusions:     exclude test files, vendor dirs, examples
//   2. Test-code suppression:     info/low findings in test files
//   3. Cross-scanner confirmation: require 2+ scanners for low/info findings
//   4. AI-grade rejection:         drop findings the grader rejected
//   5. Confidence threshold:       drop findings below configurable confidence
//   6. Severity floor:            drop findings below configurable severity
//
// Configured via `filters:` section in .apeguard.yaml.

use crate::config::FilterConfig;
use crate::find::{CanonicalFinding, Confidence, GradeVerdict, Severity};
use std::path::Path;

/// Statistics from a filter run — used for reporting and logging.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // P3/P4: FilterStats fields populated for reporting; total_before/total_after not yet read
pub struct FilterStats {
    pub path_excluded: usize,
    pub test_suppressed: usize,
    pub cross_scanner_filtered: usize,
    pub grade_rejected: usize,
    pub confidence_filtered: usize,
    pub severity_filtered: usize,
    pub total_before: usize,
    pub total_after: usize,
}

impl FilterStats {
    pub fn total_removed(&self) -> usize {
        self.path_excluded
            + self.test_suppressed
            + self.cross_scanner_filtered
            + self.grade_rejected
            + self.confidence_filtered
            + self.severity_filtered
    }
}

/// Common test-file path patterns across languages.
const TEST_FILE_PATTERNS: &[&str] = &[
    // JavaScript/TypeScript
    ".test.",
    ".spec.",
    "__tests__/",
    "/test/",
    "/tests/",
    "test/",
    "tests/",
    // Go
    "_test.go",
    // Python
    "test_",
    "_test.py",
    "conftest.py",
    // Rust
    // (Cargo tests are in tests/ dir, already covered)
    // Java
    "Test.java",
    "IT.java", // Integration test
    // General
    "/fixtures/",
    "/mocks/",
    "/__mocks__/",
];

/// Common vendor/third-party path patterns.
const VENDOR_PATTERNS: &[&str] = &[
    "vendor/",
    "node_modules/",
    "third_party/",
    "third-party/",
    "external/",
    "__pycache__/",
    ".venv/",
    "venv/",
    "dist/",
    "build/",
    "target/",
    ".next/",
    ".nuxt/",
    "bower_components/",
    "jspm_packages/",
];

/// Example/demo path patterns.
const EXAMPLE_PATTERNS: &[&str] = &[
    "examples/",
    "example/",
    "samples/",
    "sample/",
    "demo/",
    "demos/",
    "sandbox/",
    "playground/",
    "docs/examples/",
    "examples/",
];

/// Apply all FP filters and return the surviving findings plus stats.
pub fn apply_fp_filters(
    findings: Vec<CanonicalFinding>,
    config: &FilterConfig,
) -> (Vec<CanonicalFinding>, FilterStats) {
    let total_before = findings.len();
    let mut stats = FilterStats {
        total_before,
        ..Default::default()
    };

    let mut surviving = findings;

    // Stage 1: Path-based exclusions
    if config.exclude_paths_enabled {
        let (keep, removed) = partition_by(surviving, |f| {
            if is_excluded_path(&f.location.file, config) {
                stats.path_excluded += 1;
                false
            } else {
                true
            }
        });
        surviving = keep;
        let _ = removed;
    }

    // Stage 2: Test-code suppression — drop Info/Low findings in test files
    if config.suppress_test_low_severity {
        let (keep, _) = partition_by(surviving, |f| {
            if is_test_file(&f.location.file)
                && (f.severity == Severity::Info || f.severity == Severity::Low)
            {
                stats.test_suppressed += 1;
                false
            } else {
                true
            }
        });
        surviving = keep;
    }

    // Stage 3: Cross-scanner confirmation — require 2+ scanners for Info/Low
    if config.require_cross_scanner_for_low {
        let (keep, _) = partition_by(surviving, |f| {
            if f.severity == Severity::Info || f.severity == Severity::Low {
                if !f.cross_refs.is_empty() {
                    // cross_refs are OTHER scanners that confirmed — +1 for the primary scanner
                    true
                } else {
                    stats.cross_scanner_filtered += 1;
                    false
                }
            } else {
                true
            }
        });
        surviving = keep;
    }

    // Stage 4: AI-grade rejection — drop findings the grader rejected
    {
        let (keep, _) = partition_by(surviving, |f| {
            if let Some(GradeVerdict::Rejected { .. }) = &f.grade {
                stats.grade_rejected += 1;
                false
            } else {
                true
            }
        });
        surviving = keep;
    }

    // Stage 5: Confidence threshold
    if config.min_confidence > 0 {
        let (keep, _) = partition_by(surviving, |f| {
            let conf_level = match f.confidence {
                Confidence::Tentative => 0,
                Confidence::Firm => 1,
                Confidence::Certain => 2,
            };
            if conf_level < config.min_confidence {
                stats.confidence_filtered += 1;
                false
            } else {
                true
            }
        });
        surviving = keep;
    }

    // Stage 6: Severity floor
    if let Some(min_sev) = config.min_severity {
        let (keep, _) = partition_by(surviving, |f| {
            if (f.severity as u8) < min_sev {
                stats.severity_filtered += 1;
                false
            } else {
                true
            }
        });
        surviving = keep;
    }

    stats.total_after = surviving.len();
    (surviving, stats)
}

/// Partition a Vec into (keep, removed) based on a predicate.
fn partition_by<F>(
    findings: Vec<CanonicalFinding>,
    mut pred: F,
) -> (Vec<CanonicalFinding>, Vec<CanonicalFinding>)
where
    F: FnMut(&CanonicalFinding) -> bool,
{
    findings.into_iter().partition(|f| pred(f))
}

/// Check if a file path should be excluded based on configured patterns.
fn is_excluded_path(path: &Path, config: &FilterConfig) -> bool {
    let path_str = path.to_string_lossy();
    let path_lower = path_str.to_lowercase();

    // Built-in patterns
    let built_in: &[&[&str]] = if config.exclude_vendor && config.exclude_examples {
        &[TEST_FILE_PATTERNS, VENDOR_PATTERNS, EXAMPLE_PATTERNS]
    } else if config.exclude_vendor {
        &[TEST_FILE_PATTERNS, VENDOR_PATTERNS]
    } else if config.exclude_examples {
        &[TEST_FILE_PATTERNS, EXAMPLE_PATTERNS]
    } else if config.exclude_test_files {
        &[TEST_FILE_PATTERNS]
    } else {
        &[]
    };

    for patterns in built_in {
        for pat in *patterns {
            if path_lower.contains(pat) {
                return true;
            }
        }
    }

    // User-defined exclude paths
    for custom in &config.exclude_paths {
        let custom_lower = custom.to_lowercase();
        if path_lower.contains(&custom_lower) {
            return true;
        }
    }

    false
}

/// Check if a file path looks like a test file.
fn is_test_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    TEST_FILE_PATTERNS.iter().any(|p| path_str.contains(p))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
    use std::path::PathBuf;

    fn make_finding(
        id: &str,
        severity: Severity,
        file: &str,
        cross_refs: Vec<crate::find::CrossReference>,
        grade: Option<GradeVerdict>,
    ) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner: ScannerType::Semgrep,
            scanner_version: None,
            rule_id: "test-rule".into(),
            severity,
            confidence: Confidence::Firm,
            title: "Test finding".into(),
            description: "A test".into(),
            location: FindingLocation {
                file: PathBuf::from(file),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some("Fix".into()),
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs,
            grade,
            risk_score: None,
            reachable: None,
        }
    }

    fn default_config() -> FilterConfig {
        FilterConfig {
            exclude_test_files: true,
            exclude_vendor: true,
            exclude_examples: true,
            exclude_paths: vec![],
            exclude_paths_enabled: true,
            suppress_test_low_severity: true,
            require_cross_scanner_for_low: true,
            min_confidence: 0,
            min_severity: None,
        }
    }

    // --- Test stage 1: path exclusion ---

    #[test]
    fn test_excludes_vendor_paths() {
        let cfg = default_config();
        let findings = vec![
            make_finding("1", Severity::High, "src/main.rs", vec![], None),
            make_finding(
                "2",
                Severity::High,
                "node_modules/react/index.js",
                vec![],
                None,
            ),
            make_finding(
                "3",
                Severity::High,
                "vendor/github.com/lib/lib.go",
                vec![],
                None,
            ),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 1, "Should keep only non-vendor finding");
        assert_eq!(stats.path_excluded, 2);
    }

    #[test]
    fn test_excludes_test_files() {
        let cfg = default_config();
        let findings = vec![
            make_finding("1", Severity::High, "src/app.ts", vec![], None),
            make_finding("2", Severity::High, "src/utils.test.ts", vec![], None),
            make_finding("3", Severity::High, "src/handler_test.go", vec![], None),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 1, "Should keep only non-test finding");
        assert_eq!(stats.path_excluded, 2);
    }

    #[test]
    fn test_excludes_examples() {
        let cfg = default_config();
        let findings = vec![
            make_finding("1", Severity::High, "src/main.rs", vec![], None),
            make_finding("2", Severity::High, "examples/demo.py", vec![], None),
            make_finding("3", Severity::High, "sandbox/experiment.go", vec![], None),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 1);
        assert_eq!(stats.path_excluded, 2);
    }

    #[test]
    fn test_custom_exclude_paths() {
        let mut cfg = default_config();
        cfg.exclude_paths = vec!["generated/".into(), "legacy/".into()];
        let findings = vec![
            make_finding("1", Severity::High, "src/main.rs", vec![], None),
            make_finding("2", Severity::High, "generated/proto.rs", vec![], None),
            make_finding("3", Severity::High, "legacy/old.py", vec![], None),
        ];
        let (keep, _) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 1);
        assert_eq!(keep[0].id, "1");
    }

    // --- Test stage 2: test-code suppression ---

    #[test]
    fn test_suppress_low_severity_in_test_files() {
        let cfg = default_config();
        let findings = vec![
            // High severity in test file = keep (might be real vuln)
            make_finding("1", Severity::High, "src/utils.test.ts", vec![], None),
            // Low severity in test file = suppress
            make_finding("2", Severity::Low, "src/utils.test.ts", vec![], None),
            // Info in test file = suppress
            make_finding("3", Severity::Info, "src/utils.test.ts", vec![], None),
            // Low in non-test = keep (other filters apply)
            make_finding("4", Severity::Low, "src/main.rs", vec![], None),
        ];
        let (_keep, stats) = apply_fp_filters(findings, &cfg);
        // "1" is excluded by path (test file), "2" and "3" are also excluded by path
        // "4" is in src/ — not test, will be filtered by cross-scanner next
        assert_eq!(
            stats.path_excluded, 3,
            "Test path excludes High/Low/Info in test"
        );
    }

    // --- Test stage 3: cross-scanner confirmation ---

    #[test]
    fn test_cross_scanner_requirement_for_low() {
        let cfg = default_config();
        let cross_ref = crate::find::CrossReference {
            scanner: ScannerType::Gitleaks,
            rule_id: "gitleaks-key".into(),
        };
        let findings = vec![
            // Low without cross-ref = filtered
            make_finding("1", Severity::Low, "src/main.rs", vec![], None),
            // Low WITH cross-ref = kept
            make_finding("2", Severity::Low, "src/main.rs", vec![cross_ref], None),
            // High without cross-ref = kept (high severity passes through)
            make_finding("3", Severity::High, "src/main.rs", vec![], None),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        // "1" should be filtered by cross-scanner, "2" and "3" keep
        assert!(keep.iter().any(|f| f.id == "2"), "Low with cross-ref kept");
        assert!(
            keep.iter().any(|f| f.id == "3"),
            "High without cross-ref kept"
        );
        assert!(
            !keep.iter().any(|f| f.id == "1"),
            "Low without cross-ref filtered"
        );
        assert!(stats.cross_scanner_filtered >= 1);
    }

    // --- Test stage 4: AI grade rejection ---

    #[test]
    fn test_grade_rejected_filtered() {
        let cfg = default_config();
        let rejected = GradeVerdict::Rejected {
            reasoning: "False positive".into(),
            reason_category: crate::find::RejectReason::FalsePositive,
        };
        let findings = vec![
            make_finding("1", Severity::High, "src/main.rs", vec![], None),
            make_finding("2", Severity::High, "src/main.rs", vec![], Some(rejected)),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 1);
        assert_eq!(keep[0].id, "1");
        assert_eq!(stats.grade_rejected, 1);
    }

    // --- Test stage 5: confidence threshold ---

    #[test]
    fn test_confidence_threshold() {
        let mut cfg = default_config();
        cfg.min_confidence = 1; // Drop Tentative (0), keep Firm (1) and Certain (2)
        cfg.exclude_paths_enabled = false;
        cfg.suppress_test_low_severity = false;
        cfg.require_cross_scanner_for_low = false;

        let mut f1 = make_finding("1", Severity::Medium, "src/a.rs", vec![], None);
        f1.confidence = Confidence::Tentative;
        let mut f2 = make_finding("2", Severity::Medium, "src/b.rs", vec![], None);
        f2.confidence = Confidence::Firm;
        let mut f3 = make_finding("3", Severity::Medium, "src/c.rs", vec![], None);
        f3.confidence = Confidence::Certain;

        let findings = vec![f1, f2, f3];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 2, "Should drop Tentative, keep Firm + Certain");
        assert_eq!(stats.confidence_filtered, 1);
    }

    // --- Test stage 6: severity floor ---

    #[test]
    fn test_severity_floor() {
        let mut cfg = default_config();
        cfg.min_severity = Some(Severity::Medium as u8); // Drop Info and Low
        cfg.exclude_paths_enabled = false;
        cfg.suppress_test_low_severity = false;
        cfg.require_cross_scanner_for_low = false;

        let findings = vec![
            make_finding("1", Severity::Info, "src/a.rs", vec![], None),
            make_finding("2", Severity::Low, "src/b.rs", vec![], None),
            make_finding("3", Severity::Medium, "src/c.rs", vec![], None),
            make_finding("4", Severity::High, "src/d.rs", vec![], None),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 2, "Should keep Medium and High only");
        assert!(keep.iter().all(|f| f.severity >= Severity::Medium));
        assert_eq!(stats.severity_filtered, 2);
    }

    // --- Test combined: no filtering when disabled ---

    #[test]
    fn test_no_filtering_when_disabled() {
        let cfg = FilterConfig {
            exclude_test_files: false,
            exclude_vendor: false,
            exclude_examples: false,
            exclude_paths: vec![],
            exclude_paths_enabled: false,
            suppress_test_low_severity: false,
            require_cross_scanner_for_low: false,
            min_confidence: 0,
            min_severity: None,
        };
        let findings = vec![
            make_finding("1", Severity::Info, "src/utils.test.ts", vec![], None),
            make_finding("2", Severity::Low, "node_modules/x/y.js", vec![], None),
        ];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(keep.len(), 2, "All findings pass when filters disabled");
        assert_eq!(stats.total_removed(), 0);
    }

    #[test]
    fn test_empty_findings() {
        let cfg = default_config();
        let findings: Vec<CanonicalFinding> = vec![];
        let (keep, stats) = apply_fp_filters(findings, &cfg);
        assert!(keep.is_empty());
        assert_eq!(stats.total_before, 0);
        assert_eq!(stats.total_after, 0);
    }

    #[test]
    fn test_stats_total_removed() {
        let cfg = default_config();
        let findings = vec![
            make_finding("1", Severity::Low, "src/main.rs", vec![], None),
            make_finding("2", Severity::High, "src/main.rs", vec![], None),
        ];
        let (_, stats) = apply_fp_filters(findings, &cfg);
        assert_eq!(stats.total_before, 2);
        assert_eq!(
            stats.total_removed() + stats.total_after,
            stats.total_before
        );
    }
}

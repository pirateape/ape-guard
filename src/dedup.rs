// ApeGuard Deduplication Module
// Removes duplicate findings across scanners.
// A finding is a duplicate if it has the same file, line, AND rule_id.
// (same file+line alone is insufficient — two different CVEs can appear at the same location)
use crate::find::CanonicalFinding;
use std::collections::HashSet;

/// Deduplicate findings across all scanners.
/// Priority: prefer findings from higher-confidence scanners.
pub fn deduplicate(findings: Vec<CanonicalFinding>) -> Vec<CanonicalFinding> {
    let mut seen: HashSet<(String, Option<u32>, String)> = HashSet::new();
    let mut deduped = Vec::new();

    for finding in findings {
        match finding.location.line {
            Some(line) => {
                let key = (
                    finding.location.file.to_string_lossy().to_string(),
                    Some(line),
                    finding.rule_id.clone(),
                );
                if seen.insert(key) {
                    deduped.push(finding);
                }
                // Skip only exact duplicates: same file + same line + same rule
            }
            None => {
                // No line info = always keep (can't confirm duplicate location)
                deduped.push(finding);
            }
        }
    }

    deduped
}

/// Cross-reference findings across scanners.
/// Links findings from different scanners that reference the same file/line.
/// Each finding gets cross-references to ALL OTHER findings at the same location.
pub fn cross_reference(findings: &mut [CanonicalFinding]) {
    use std::collections::HashMap;

    let mut location_map: HashMap<(String, Option<u32>), Vec<usize>> = HashMap::new();

    // Group findings by (file, line) — intentionally without rule_id so
    // different rules at the same location still cross-reference each other
    for (i, finding) in findings.iter().enumerate() {
        let key = (
            finding.location.file.to_string_lossy().to_string(),
            finding.location.line,
        );
        location_map.entry(key).or_default().push(i);
    }

    // Cross-reference groups: each finding links to all others at same location
    for indices in location_map.values() {
        if indices.len() < 2 {
            continue;
        }

        for &i in indices {
            for &j in indices {
                if i != j {
                    // Avoid duplicate refs (same scanner + rule combination)
                    let already = findings[i].cross_refs.iter().any(|r| {
                        r.scanner == findings[j].scanner && r.rule_id == findings[j].rule_id
                    });
                    if !already {
                        findings[i].cross_refs.push(crate::find::CrossReference {
                            scanner: findings[j].scanner.clone(),
                            rule_id: findings[j].rule_id.clone(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::*;

    fn make_finding(
        id: &str,
        file: &str,
        line: Option<u32>,
        scanner: ScannerType,
    ) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner,
            scanner_version: None,
            rule_id: "test-rule".into(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: "Test".into(),
            description: "Test finding".into(),
            location: FindingLocation {
                file: std::path::PathBuf::from(file),
                line,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
        }
    }

    #[test]
    fn test_dedup_removes_duplicates() {
        let findings = vec![
            make_finding("a", "file.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file.py", Some(10), ScannerType::Semgrep), // same location
            make_finding("c", "file.py", Some(20), ScannerType::TrivyVuln),
        ];

        let deduped = deduplicate(findings);
        // First finding at (file.py, 10) is kept, second is removed
        assert_eq!(deduped.len(), 2);
        assert_eq!(deduped[0].id, "a");
        assert_eq!(deduped[1].id, "c");
    }

    #[test]
    fn test_dedup_different_files() {
        let findings = vec![
            make_finding("a", "file1.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file2.py", Some(10), ScannerType::Semgrep),
        ];

        let deduped = deduplicate(findings);
        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_dedup_same_file_different_lines() {
        let findings = vec![
            make_finding("a", "file.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file.py", Some(20), ScannerType::Semgrep),
        ];

        let deduped = deduplicate(findings);
        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_dedup_no_line_number() {
        let findings = vec![
            make_finding("a", "file.py", None, ScannerType::Gitleaks),
            make_finding("b", "file.py", None, ScannerType::Semgrep),
        ];

        let deduped = deduplicate(findings);
        assert_eq!(deduped.len(), 2); // both kept if None
    }

    #[test]
    fn test_empty_findings() {
        let findings: Vec<CanonicalFinding> = vec![];
        let deduped = deduplicate(findings);
        assert!(deduped.is_empty());
    }

    #[test]
    fn test_cross_reference_links_related() {
        let mut findings = vec![
            make_finding("a", "file.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file.py", Some(10), ScannerType::Semgrep),
        ];

        cross_reference(&mut findings);

        // Both findings get cross-ref to each other
        assert_eq!(findings[0].cross_refs.len(), 1);
        assert_eq!(findings[0].cross_refs[0].scanner, ScannerType::Semgrep);

        assert_eq!(findings[1].cross_refs.len(), 1);
        assert_eq!(findings[1].cross_refs[0].scanner, ScannerType::Gitleaks);
    }

    #[test]
    fn test_cross_reference_three_scanners() {
        let mut findings = vec![
            make_finding("a", "file.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file.py", Some(10), ScannerType::Semgrep),
            make_finding("c", "file.py", Some(10), ScannerType::TrivyVuln),
        ];

        cross_reference(&mut findings);

        // Each finding gets cross-ref to all other 2 scanners
        assert_eq!(findings[0].cross_refs.len(), 2);
        assert_eq!(findings[1].cross_refs.len(), 2);
        assert_eq!(findings[2].cross_refs.len(), 2);
    }

    #[test]
    fn test_cross_reference_no_self_ref() {
        let mut findings = vec![make_finding(
            "a",
            "file.py",
            Some(10),
            ScannerType::Gitleaks,
        )];

        cross_reference(&mut findings);

        // Single finding should have no cross-references
        assert!(findings[0].cross_refs.is_empty());
    }

    #[test]
    fn test_cross_reference_different_location_no_link() {
        let mut findings = vec![
            make_finding("a", "file.py", Some(10), ScannerType::Gitleaks),
            make_finding("b", "file.py", Some(20), ScannerType::Semgrep),
        ];

        cross_reference(&mut findings);

        // Different lines = no cross-refs
        assert!(findings[0].cross_refs.is_empty());
        assert!(findings[1].cross_refs.is_empty());
    }
}

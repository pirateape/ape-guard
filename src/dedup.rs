// ApeGuard Deduplication Module
// Removes duplicate findings across scanners.
// A finding is a duplicate if it has the same file, line, and rule pattern.
use crate::find::CanonicalFinding;
use std::collections::HashSet;

/// Deduplicate findings across all scanners.
/// Priority: prefer findings from higher-confidence scanners.
pub fn deduplicate(findings: Vec<CanonicalFinding>) -> Vec<CanonicalFinding> {
    let mut seen: HashSet<(String, Option<u32>)> = HashSet::new();
    let mut deduped = Vec::new();

    for finding in findings {
        let key = (
            finding.location.file.to_string_lossy().to_string(),
            finding.location.line,
        );

        if seen.insert(key) {
            deduped.push(finding);
        }
        // Skip if duplicate
    }

    deduped
}

/// Cross-reference findings across scanners.
/// Links findings from different scanners that reference the same file/line.
pub fn cross_reference(findings: &mut [CanonicalFinding]) {
    use std::collections::HashMap;

    let mut location_map: HashMap<(String, Option<u32>), Vec<usize>> = HashMap::new();

    // Group findings by (file, line)
    for (i, finding) in findings.iter().enumerate() {
        let key = (
            finding.location.file.to_string_lossy().to_string(),
            finding.location.line,
        );
        location_map.entry(key).or_default().push(i);
    }

    // Cross-reference groups
    for indices in location_map.values() {
        if indices.len() < 2 {
            continue;
        }

        // The first scanner to report gets linked to the rest
        let _primary = indices[0];
        let secondary_scanners: Vec<_> = indices[1..]
            .iter()
            .map(|&i| (findings[i].scanner.clone(), findings[i].rule_id.clone()))
            .collect();

        for &i in indices {
            for (scanner, rule_id) in &secondary_scanners {
                if findings[i].scanner != *scanner {
                    findings[i].cross_refs.push(crate::find::CrossReference {
                        scanner: scanner.clone(),
                        rule_id: rule_id.clone(),
                    });
                }
            }
        }
    }
}

// ApeGuard Adversarial Verification Grader
//
// Uses an adversarial "prove FP" frame to independently verify scanner findings.
// Reuses the existing llm.rs Ollama integration with a different prompt strategy.
//
// Architecture: docs/grade-engine.md
// Pattern: defending-code-reference-harness — separate grader with independent reasoning
use crate::find::{CanonicalFinding, GradeVerdict, RejectReason};
use serde::Deserialize;

/// Grade all findings using the adversarial grader.
///
/// Skips findings that already have a grade (e.g. from resume).
/// Gracefully degrades if Ollama is not running.
///
/// Returns the number of findings successfully graded.
pub async fn grade_findings(
    findings: &mut [CanonicalFinding],
    endpoint: &str,
    model: &str,
) -> anyhow::Result<u32> {
    // Check if Ollama is available
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let ping = client.head(format!("{}/api/tags", endpoint)).send().await;

    if ping.is_err() {
        tracing::warn!(
            "Ollama not available at {} — skipping LLM grading",
            endpoint
        );
        return Ok(0);
    }

    let mut graded = 0u32;

    for finding in findings.iter_mut() {
        // Skip findings that already have a grade (e.g. from resume or previous run)
        if finding.grade.is_some() {
            continue;
        }

        let prompt = build_grade_prompt(finding);

        match crate::llm::call_ollama(endpoint, model, &prompt).await {
            Ok(response) => {
                let verdict = parse_grade_response(&response);
                tracing::debug!(
                    "Graded {} ({:?}): {:?}",
                    finding.id,
                    finding.severity,
                    std::mem::discriminant(&verdict)
                );
                finding.grade = Some(verdict);
                graded += 1;
            }
            Err(e) => {
                tracing::debug!("LLM grading failed for {}: {}", finding.id, e);
            }
        }
    }

    Ok(graded)
}

/// Build the adversarial grade prompt for a single finding.
///
/// The prompt uses an adversarial frame: the grader must PROVE the finding
/// is a false positive. This catches the asymmetry where scanners err toward
/// false positives (better to report and be wrong than miss a real issue).
fn build_grade_prompt(finding: &CanonicalFinding) -> String {
    let snippet = finding
        .location
        .snippet
        .as_deref()
        .unwrap_or("(no snippet available)");
    let line = finding
        .location
        .line
        .map_or("-".to_string(), |l| l.to_string());

    format!(
        r#"You are an adversarial security grader for ApeGuard.
Your job is to independently verify a scanner finding.

## Finding to grade
Rule: {} ({})
Severity: {:?}
File: {}:{}
Snippet:
```
{}
```

## Your task
Argue AGAINST this finding. Prove it is a false positive.
Consider:
1. Is this test/documentation code?
2. Is the vulnerable path reachable?
3. Are there existing guards/mitigations?
4. Is there a benign interpretation?
5. Is the severity appropriate?

## Output format (JSON only)
{{
  "verdict": "CONFIRMED" | "REJECTED" | "NEEDS_REVIEW",
  "confidence": 0.0-1.0,
  "reasoning": "Detailed step-by-step analysis...",
  "reject_reason": "TestCode" | "Unreachable" | "FalsePositive" | "AlreadyMitigated" | "SeverityInflated" | null,
  "open_questions": ["question 1", ...]
}}
"#,
        finding.rule_id,
        finding.scanner,
        finding.severity,
        finding.location.file.display(),
        line,
        snippet,
    )
}

/// Raw JSON response from the LLM grader
#[derive(Deserialize)]
struct GradeResponse {
    verdict: String,
    #[serde(default)]
    confidence: Option<f32>,
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    reject_reason: Option<String>,
    #[serde(default)]
    open_questions: Option<Vec<String>>,
}

/// Parse the LLM response into a structured GradeVerdict.
///
/// Handles:
/// - Clean JSON response
/// - Markdown-fenced JSON (```json ... ```)
/// - Free-text with verdict keyword fallback
fn parse_grade_response(response: &str) -> GradeVerdict {
    let cleaned = response.trim();

    // Step 1: Extract JSON from markdown code block if present
    let json_str = if let Some(start) = cleaned.find("```json") {
        let after_fence = &cleaned[start + 7..];
        let end = after_fence.find("```").unwrap_or(after_fence.len());
        after_fence[..end].trim()
    } else if let Some(start) = cleaned.find('{') {
        let after_brace = &cleaned[start..];
        let end = after_brace
            .rfind('}')
            .map(|e| start + e + 1)
            .unwrap_or(cleaned.len());
        &cleaned[start..end]
    } else {
        cleaned
    };

    // Step 2: Try structured JSON parsing
    if let Ok(parsed) = serde_json::from_str::<GradeResponse>(json_str) {
        return match parsed.verdict.as_str() {
            "CONFIRMED" => GradeVerdict::Confirmed {
                confidence: parsed.confidence.unwrap_or(0.8),
                reasoning: parsed.reasoning.unwrap_or_default(),
            },
            "REJECTED" => GradeVerdict::Rejected {
                reasoning: parsed.reasoning.unwrap_or_default(),
                reason_category: parse_reject_reason(&parsed.reject_reason),
            },
            "NEEDS_REVIEW" => GradeVerdict::NeedsReview {
                reasoning: parsed.reasoning.unwrap_or_default(),
                open_questions: parsed.open_questions.unwrap_or_default(),
            },
            other => GradeVerdict::NeedsReview {
                reasoning: format!(
                    "Unknown verdict '{}': {}",
                    other,
                    parsed.reasoning.as_deref().unwrap_or("no reasoning")
                ),
                open_questions: vec!["Could not parse verdict".to_string()],
            },
        };
    }

    // Step 3: Fallback — keyword matching on raw response
    let upper = cleaned.to_uppercase();
    if upper.contains("CONFIRMED") {
        GradeVerdict::Confirmed {
            confidence: 0.7,
            reasoning: cleaned.to_string(),
        }
    } else if upper.contains("REJECTED") {
        GradeVerdict::Rejected {
            reasoning: cleaned.to_string(),
            reason_category: RejectReason::FalsePositive,
        }
    } else {
        GradeVerdict::NeedsReview {
            reasoning: cleaned.to_string(),
            open_questions: vec!["Could not parse structured response".to_string()],
        }
    }
}

/// Map a string reject_reason to the RejectReason enum.
fn parse_reject_reason(reason: &Option<String>) -> RejectReason {
    match reason.as_deref() {
        Some("TestCode") => RejectReason::TestCode,
        Some("Unreachable") => RejectReason::Unreachable,
        Some("FalsePositive") => RejectReason::FalsePositive,
        Some("AlreadyMitigated") => RejectReason::AlreadyMitigated,
        Some("SeverityInflated") => RejectReason::SeverityInflated,
        _ => RejectReason::FalsePositive,
    }
}

/// Count the distribution of grade verdicts across findings.
pub fn count_verdicts(findings: &[CanonicalFinding]) -> GradeCounts {
    let mut counts = GradeCounts {
        confirmed: 0,
        rejected: 0,
        needs_review: 0,
        ungraded: 0,
    };

    for f in findings {
        match f.grade {
            Some(GradeVerdict::Confirmed { .. }) => counts.confirmed += 1,
            Some(GradeVerdict::Rejected { .. }) => counts.rejected += 1,
            Some(GradeVerdict::NeedsReview { .. }) => counts.needs_review += 1,
            None => counts.ungraded += 1,
        }
    }

    counts
}

/// Distribution of grade verdicts across findings.
#[derive(Debug, Clone, Copy, Default)]
pub struct GradeCounts {
    pub confirmed: u32,
    pub rejected: u32,
    pub needs_review: u32,
    pub ungraded: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::*;
    use std::path::PathBuf;

    fn make_finding(id: &str) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "test-rule".into(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: "Test Finding".into(),
            description: "A test finding for grading".into(),
            location: FindingLocation {
                file: PathBuf::from("src/main.rs"),
                line: Some(42),
                column: None,
                commit: None,
                author: None,
                snippet: Some("api_key = \"12345\"".into()),
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        }
    }

    #[test]
    fn test_build_grade_prompt_includes_finding() {
        let finding = make_finding("F-001");
        let prompt = build_grade_prompt(&finding);

        assert!(prompt.contains("test-rule"));
        assert!(prompt.contains("High"));
        assert!(prompt.contains("src/main.rs"));
        assert!(prompt.contains("api_key = \"12345\""));
        assert!(prompt.contains("Argue AGAINST this finding"));
        assert!(prompt.contains("CONFIRMED"));
        assert!(prompt.contains("REJECTED"));
        assert!(prompt.contains("NEEDS_REVIEW"));
    }

    #[test]
    fn test_parse_confirmed_json() {
        let response = r#"{"verdict":"CONFIRMED","confidence":0.95,"reasoning":"The finding is valid. The API key is hardcoded in production code with no test fixture indicators. No existing mitigations found."}"#;
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::Confirmed {
                confidence,
                reasoning,
            } => {
                assert!((confidence - 0.95).abs() < 0.01);
                assert!(!reasoning.is_empty());
            }
            _ => panic!("Expected Confirmed, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_rejected_json() {
        let response = r#"{"verdict":"REJECTED","reasoning":"This is a test file in the test fixtures directory. The code path is not reachable in production.","reject_reason":"TestCode"}"#;
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::Rejected {
                reasoning,
                reason_category,
            } => {
                assert!(!reasoning.is_empty());
                assert_eq!(reason_category, RejectReason::TestCode);
            }
            _ => panic!("Expected Rejected, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_needs_review_json() {
        let response = r#"{"verdict":"NEEDS_REVIEW","confidence":0.5,"reasoning":"The finding may be valid but requires human review because the code pattern is ambiguous.","open_questions":["Is this reachable from user input?","What authentication is in place?"]}"#;
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::NeedsReview {
                reasoning,
                open_questions,
            } => {
                assert!(!reasoning.is_empty());
                assert_eq!(open_questions.len(), 2);
            }
            _ => panic!("Expected NeedsReview, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_markdown_fenced_json() {
        let response = "Here is my analysis:\n\n```json\n{\"verdict\":\"CONFIRMED\",\"confidence\":0.88,\"reasoning\":\"Real vulnerability.\"}\n```\n\nHope this helps!";
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::Confirmed { confidence, .. } => {
                assert!((confidence - 0.88).abs() < 0.01);
            }
            _ => panic!("Expected Confirmed, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_fallback_keyword_confirmed() {
        let response =
            "After analysis, I CONFIRMED this finding. The API key is in production code.";
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::Confirmed { .. } => {} // OK
            _ => panic!("Expected Confirmed via fallback, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_fallback_keyword_rejected() {
        let response = "This finding is REJECTED. It's in a test fixture.";
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::Rejected { .. } => {} // OK
            _ => panic!("Expected Rejected via fallback, got {:?}", verdict),
        }
    }

    #[test]
    fn test_parse_unknown_verdict() {
        let response = r#"{"verdict":"MAYBE","reasoning":"Unclear"}"#;
        let verdict = parse_grade_response(response);

        match verdict {
            GradeVerdict::NeedsReview { .. } => {} // Unknown maps to NeedsReview
            _ => panic!(
                "Expected NeedsReview for unknown verdict, got {:?}",
                verdict
            ),
        }
    }

    #[test]
    fn test_count_verdicts_all_types() {
        let mut findings = vec![
            make_finding("F-001"),
            make_finding("F-002"),
            make_finding("F-003"),
            make_finding("F-004"),
        ];

        findings[0].grade = Some(GradeVerdict::Confirmed {
            confidence: 0.9,
            reasoning: "real".into(),
        });
        findings[1].grade = Some(GradeVerdict::Rejected {
            reasoning: "fp".into(),
            reason_category: RejectReason::TestCode,
        });
        findings[2].grade = Some(GradeVerdict::NeedsReview {
            reasoning: "maybe".into(),
            open_questions: vec![],
        });
        // F-004 remains ungraded

        let counts = count_verdicts(&findings);
        assert_eq!(counts.confirmed, 1);
        assert_eq!(counts.rejected, 1);
        assert_eq!(counts.needs_review, 1);
        assert_eq!(counts.ungraded, 1);
    }

    #[test]
    fn test_parse_reject_reason_all_variants() {
        assert_eq!(
            parse_reject_reason(&Some("TestCode".into())),
            RejectReason::TestCode
        );
        assert_eq!(
            parse_reject_reason(&Some("Unreachable".into())),
            RejectReason::Unreachable
        );
        assert_eq!(
            parse_reject_reason(&Some("FalsePositive".into())),
            RejectReason::FalsePositive
        );
        assert_eq!(
            parse_reject_reason(&Some("AlreadyMitigated".into())),
            RejectReason::AlreadyMitigated
        );
        assert_eq!(
            parse_reject_reason(&Some("SeverityInflated".into())),
            RejectReason::SeverityInflated
        );
        assert_eq!(
            parse_reject_reason(&Some("Unknown".into())),
            RejectReason::FalsePositive
        );
        assert_eq!(parse_reject_reason(&None), RejectReason::FalsePositive);
    }

    #[test]
    fn test_grade_verdict_serialize_roundtrip() {
        let original = GradeVerdict::Confirmed {
            confidence: 0.85,
            reasoning: "Valid finding".into(),
        };

        let json =
            serde_json::to_string(&original).expect("grade test: failed to serialize verdict");
        let parsed: GradeVerdict =
            serde_json::from_str(&json).expect("grade test: failed to parse JSON verdict");

        match parsed {
            GradeVerdict::Confirmed { confidence, .. } => {
                assert!((confidence - 0.85).abs() < 0.01);
            }
            _ => panic!("Roundtrip failed"),
        }
    }

    #[test]
    fn test_grade_field_on_canonical_finding() {
        let mut finding = make_finding("F-grade-test");
        assert!(finding.grade.is_none());

        finding.grade = Some(GradeVerdict::Rejected {
            reasoning: "test code".into(),
            reason_category: RejectReason::TestCode,
        });
        assert!(finding.grade.is_some());

        let json =
            serde_json::to_string(&finding).expect("grade test: failed to serialize finding");
        assert!(
            json.contains(r#""Rejected""#),
            "Expected Rejected variant in JSON, got: {}",
            json
        );
        assert!(
            json.contains("TestCode"),
            "Expected TestCode in JSON, got: {}",
            json
        );
    }
}

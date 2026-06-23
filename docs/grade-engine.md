# Option C: Adversarial Verification Grader

> Status: **Design Phase** | Priority: Medium | Depends on: found_findings.jsonl (done in Option A)

## Motivation

ApeGuard's 7 scanner layers produce findings with known false-positive rates:

- **Gitleaks**: ~5-15% FP (noisy on test fixtures, documentation examples)
- **Semgrep**: ~10-30% FP (context-dependent rules)
- **Trivy**: ~5% FP (version matching errors)
- **Nuclei**: ~20-40% FP (live service behavior variance)

Current state: all findings are treated as equally valid. No independent verification step exists.

**Pattern from defending-code-reference-harness**: The grade agent runs in a separate container with only the PoC input — it cannot see the find-agent's reasoning. This prevents reward-hacking and provides true independent verification.

## Architecture

```
Findings from scanners (CanonicalFinding[])
         │
         ▼
┌─────────────────────────────────┐
│  GRADE FILTER                   │
│  - Skip already-confirmed       │
│  - Severity threshold filter    │
│  - Max findings per run limit   │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│  LLM GRADER (per finding)       │
│  "Prove this is NOT a finding"  │
│  Output: verdict + reasoning    │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│  VERDICT AGGREGATOR             │
│  - CONFIRMED → keep, adjust     │
│    confidence score             │
│  - REJECTED → mark as FP,      │
│    exclude from report          │
│  - NEEDS_REVIEW → flag for      │
│    human review                 │
└────────────┬────────────────────┘
             │
             ▼
      Report generation
      (incorporates grades)
```

## Key Design Decisions

### 1. Adversarial Frame (from defending-code-reference-harness)

The grader prompt uses **"guilty until proven innocent"** framing:

```
You are an adversarial security reviewer. Your job is to PROVE
that the following finding is a FALSE POSITIVE. Challenge every
assumption. Look for:

1. Is the finding in test code, documentation, or example files?
2. Is the vulnerable code path reachable in production?
3. Are there existing mitigations the scanner missed?
4. Is the severity inflated?
5. Could there be a benign explanation?

If you cannot prove it's a false positive after thorough analysis,
classify it as CONFIRMED. If you're uncertain, classify as NEEDS_REVIEW.
```

This forces the grader to argue against the finding — catching the asymmetry where scanners err toward false positives (better to report and be wrong than miss a real issue).

### 2. Per-Finding vs Batch Grading

| Approach                      | Pros                                             | Cons                             |
| ----------------------------- | ------------------------------------------------ | -------------------------------- |
| **Per-finding** (recommended) | Independent, no cascading errors, parallelizable | Higher LLM cost, slower          |
| **Batch**                     | Cheaper, faster                                  | Contextual bias between findings |

**Recommendation**: Per-finding for critical/high severity. Batch low/info severity.

### 3. Integration Points

- **CLI flag**: `--grade` to enable (like `--resume`)
- **Config field**: `[grade] enabled = true` in ApeGuard config
- **Output field**: `CanonicalFinding.grade` added as `Option<GradeVerdict>`

### 4. Where It Runs

The grader reuses the existing `llm.rs` Ollama integration pattern but with a different prompt and more context (the actual finding's code snippet + surrounding file lines).

## Data Model

```rust
/// Verdict from the adversarial grader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradeVerdict {
    /// Grader could not disprove the finding
    Confirmed { confidence: f32, reasoning: String },
    /// Grader proved it's a false positive
    Rejected { reasoning: String, reason_category: RejectReason },
    /// Grader could not determine either way
    NeedsReview { reasoning: String, open_questions: Vec<String> },
}

/// Why a finding was rejected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RejectReason {
    TestCode,          /// In test fixtures, mocks, or examples
    Unreachable,       /// Code path not reachable in production
    FalsePositive,     /// Scanner logic error / version mismatch
    AlreadyMitigated,  /// Existing control handles this
    SeverityInflated,  /// Real issue but severity is wrong
}

// Added to CanonicalFinding:
pub struct CanonicalFinding {
    // ... existing fields ...
    pub grade: Option<GradeVerdict>,  /// NEW: AI verification grade
}
```

## Prompt for the Grader

```text
You are an adversarial security grader for ApeGuard.
Your job is to independently verify a scanner finding.

## Finding to grade
Rule: {rule_id} ({scanner})
Severity: {severity}
File: {file}:{line}
Snippet:
```

{snippet}

```

## Context (surrounding code, 5 lines before and after)
```

{context}

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
{
  "verdict": "CONFIRMED" | "REJECTED" | "NEEDS_REVIEW",
  "confidence": 0.0-1.0,
  "reasoning": "Detailed step-by-step analysis...",
  "reject_reason": "TestCode" | "Unreachable" | "FalsePositive" | "AlreadyMitigated" | "SeverityInflated" | null,
  "open_questions": ["question 1", ...]   // only for NEEDS_REVIEW
}
```

## Streaming to found_findings.jsonl

The grade verdict is appended to the existing JSONL entry for each finding:

```jsonl
{"scanner":"gitleaks","finding_id":"F-001","status":"found","count":1,...}
{"scanner":"gitleaks","finding_id":"F-001","status":"grade","verdict":"CONFIRMED","confidence":0.92,"reasoning":"..."}
```

The `load_completed_scanners()` function (from Option A) naturally skips already-graded findings on resume.

## Implementation Plan

### Phase 1: Core Grader Module (`src/grade.rs`)

- `GradeVerdict` enum + `RejectReason` enum
- `grade_finding()` async function — calls Ollama with adversarial prompt
- `grade_findings()` batch orchestrator
- Test: unit tests with mock Ollama responses

### Phase 2: Integration

- Add `grade: Option<GradeVerdict>` to `CanonicalFinding`
- Add `--grade` flag to CLI
- Wire into scan pipeline (runs after all scanners, before report)
- Write grade verdicts to `found_findings.jsonl`

### Phase 3: Report Integration

- Filter `REJECTED` findings from main report (optional: include in appendix)
- Show confidence adjustments in report
- Grade summary section: X confirmed, Y rejected, Z needs review

### Phase 4: Advanced

- Per-scanner FP rate tracking (learn which scanners produce most rejects)
- Auto-learned severity adjustments based on historical grade patterns
- Grade cache (don't re-grade the same finding pattern twice)

## Verification

```
# Run scan with grading enabled
cargo run -- scan /path/to/target --grade

# Expected:
# [1/7] gitleaks ... 3 findings
# [2/7] semgrep ... 5 findings
# ...
# [8/8] grading ... 8 findings graded (5 confirmed, 2 rejected, 1 needs review)
# Report written to .apeguard/reports/
```

## Open Questions

1. **LLM cost**: At ~500 tokens per finding, 100 findings = 50K tokens. Acceptable for a security audit tool. Consider gating with `--grade --max-grade-findings 20`.
2. **Offline mode**: Falls back to skip gracefully (same as existing `llm.rs` pattern) if Ollama unavailable.
3. **False negative risk**: A grader that rejects a real finding is worse than no grader. The adversarial frame mitigates this — the grader must prove FP, not just assert it.

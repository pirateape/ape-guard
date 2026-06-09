# Option D: Multi-Agent Scoring Pipeline

> Status: **Design Phase** | Priority: Medium | Depends on: Option C (grade engine) OR standalone

## Motivation

ApeGuard currently scores findings in two isolated ways:
1. **Severity** (Critical/High/Medium/Low/Info) — assigned by the scanner, not verified
2. **Attack chains** (`chain.rs`) — groups findings by proximity and pattern matches

Missing: a unified risk score that considers ALL dimensions simultaneously.

**Pattern from defending-code-reference-harness**: The judge agent reads each crash against all prior crashes and decides NEW/DUP_BETTER/DUP_SKIP — a holistic scoring step that considers context beyond what individual agents have. Similarly, ApeGuard needs a scoring phase that considers findings holistically.

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    SCORING PIPELINE                       │
│                                                          │
│  INPUTS:                                                 │
│    • CanonicalFindings (from all scanners)                │
│    • AttackChains (from chain.rs)                         │
│    • GradeVerdicts (from Option C, or None)               │
│    • ZTScorecard (existing)                               │
│    • Cross-references (from dedup.rs)                     │
│                                                          │
│  ┌──────────────────────────────────────────────────┐    │
│  │  SCORING AGENTS (each scores one dimension)       │    │
│  │                                                    │    │
│  │  1. SEVERITY AGENT    → Base severity weight       │    │
│  │  2. CONFIDENCE AGENT  → Scanner reliability weight │    │
│  │  3. CONTEXT AGENT     → Proximity to critical code  │    │
│  │  4. CHAIN AGENT       → Attack chain position      │    │
│  │  5. ZT AGENT          → Zero Trust pillar impact   │    │
│  │  6. GRADE AGENT       → AI verification confidence │    │
│  └──────────────────────────────────────────────────┘    │
│                      │                                    │
│                      ▼                                    │
│  ┌──────────────────────────────────────────────────┐    │
│  │  WEIGHTED AGGREGATOR                              │    │
│  │  Combines all scores → unified risk_score per     │    │
│  │  finding + overall scan health score              │    │
│  └──────────────────────────────────────────────────┘    │
│                      │                                    │
│                      ▼                                    │
│  OUTPUT: ScoringReport with per-finding + overall scores  │
└──────────────────────────────────────────────────────────┘
```

## Scoring Dimensions

Each dimension produces a score from 0.0 (no risk) to 1.0 (maximum risk).

### 1. Severity Score (0.0–1.0)

Maps scanner severity to a numeric score:

| Severity | Score | Weight |
|----------|-------|--------|
| Critical | 1.00 | 3.0x |
| High | 0.75 | 2.0x |
| Medium | 0.50 | 1.0x |
| Low | 0.25 | 0.5x |
| Info | 0.10 | 0.25x |

### 2. Confidence Score (0.0–1.0)

Based on scanner type + cross-reference count:

| Signal | Score |
|--------|-------|
| Single scanner, no cross-refs | 0.4 |
| Single scanner, has cross-refs | 0.6 |
| 2+ scanners agree | 0.8 |
| 3+ scanners agree | 0.95 |
| Confirmed by AI grader | 0.98 |
| Rejected by AI grader | 0.05 |

### 3. Context Score (0.0–1.0)

How close is the finding to critical code paths?

| Location | Score |
|----------|-------|
| Test directory, docs, examples | 0.1 |
| Utility/library code | 0.3 |
| API handler, controller | 0.6 |
| Auth/AuthZ code | 0.8 |
| Security-critical (crypto, TLS) | 0.95 |
| In production-only paths | 0.7 |

### 4. Chain Score (0.0–1.0)

Is this finding part of an attack chain?

| Condition | Score |
|-----------|-------|
| Not in any chain | 0.3 |
| In 1 chain | 0.7 |
| In 2+ chains | 0.9 |
| Chain risk multiplier applied | x1.0–2.5 |

### 5. ZT Pillar Score (0.0–1.0)

How many Zero Trust pillars does this finding affect?

| Pillars affected | Score |
|-----------------|-------|
| 0 | 0.1 |
| 1 | 0.4 |
| 2 | 0.6 |
| 3+ | 0.85 |

### 6. Grade Score (0.0–1.0)

From Option C's adversarial grader (or defaults):

| Grade Verdict | Score |
|--------------|-------|
| Confirmed (confidence ≥ 0.8) | 0.9 |
| Confirmed (confidence < 0.8) | 0.7 |
| Needs Review | 0.5 |
| Rejected | 0.1 |
| Not graded | 0.5 (neutral) |

## Unified Risk Score

```rust
pub struct UnifiedRiskScore {
    /// Overall risk score for a single finding (0.0 = no risk, 1.0 = max)
    pub overall: f32,
    /// Per-dimension breakdown
    pub dimensions: RiskDimensions,
    /// Confidence in this score (0.0-1.0)
    pub confidence: f32,
}

pub struct RiskDimensions {
    pub severity: f32,
    pub confidence: f32,
    pub context: f32,
    pub chain: f32,
    pub zt_pillars: f32,
    pub grade: f32,
}

// Weighted sum:
// overall = (severity * W_sev + confidence * W_conf + context * W_ctx
//            + chain * W_chain + zt_pillars * W_zt + grade * W_grade)
//           / (sum of weights)
//
// Default weights (configurable):
// W_sev   = 0.25
// W_conf  = 0.20
// W_ctx   = 0.15
// W_chain = 0.15
// W_zt    = 0.10
// W_grade = 0.15
```

## Data Model

```rust
/// Added to CanonicalFinding:
pub struct CanonicalFinding {
    // ... existing fields ...
    pub risk_score: Option<UnifiedRiskScore>,  // NEW
}

/// Overall scan health score
pub struct ScanHealthScore {
    /// 0-1000 scale
    pub overall: u32,
    /// Per-dimension scores
    pub dimensions: ScanHealthDimensions,
    /// Trend (if historical data available)
    pub trend: Option<ScoreTrend>,
}

pub struct ScanHealthDimensions {
    pub total_risk_burden: f32,      // Sum of all finding risk scores
    pub critical_finding_density: f32, // % of findings that are critical
    /// Average risk per scanner
    pub scanner_risk: HashMap<String, f32>,
    /// ZT maturity score (already exists in ZTScorecard)
    pub zt_maturity: u32,
}
```

## Implementation Plan

### Phase 1: Scoring Core (`src/score.rs`)
- `UnifiedRiskScore` + `RiskDimensions` structs
- `ScoreWeights` config (with defaults)
- `compute_finding_risk()` function — takes a finding + context, returns unified score
- `compute_scan_health()` function — aggregates all findings into overall metrics
- Tests: verify scoring formulas with known inputs

### Phase 2: Dimension Agents
- Context heuristic (file path analysis)
- Chain integration (check if finding is in existing attack chains)
- ZT pillar counting
- Confidence calculation from cross-refs + scanner identity

### Phase 3: Integration
- Add `risk_score` to `CanonicalFinding`
- Wire into scan pipeline (runs after dedup + cross-ref + chain analysis)
- Add `--score` / `--risk-score` CLI flag
- Include scores in report output

### Phase 4: Report Integration
- Findings sorted by risk score (highest first)
- Risk score shown in finding details
- Scan health score in summary
- Optional: risk score distribution chart in HTML report

## Verification

```rust
// Test: single critical finding, no cross-refs
let finding = make_critical_finding("RCE in handler");
let context = ScoringContext { chains: vec![], grade: None };
let score = compute_finding_risk(&finding, &context);
assert!(score.overall > 0.5);  // Critical alone should score high
assert!(score.dimensions.severity == 1.0);

// Test: low-severity with AI confirmation
let finding = make_low_finding("Info log");
let context = ScoringContext { chains: vec![], grade: Some(GradeVerdict::Rejected) };
let score = compute_finding_risk(&finding, &context);
assert!(score.overall < 0.3);  // Rejected + Low = very low score
```

## Open Questions

1. **Weight calibration**: Default weights need to be validated against real scan data. Consider a `--calibrate` mode that runs weights across common vulnerability corpora.
2. **LLM-assisted scoring**: For the context dimension, an LLM could analyze the code path more deeply (like Option C's grader). Keep this optional to avoid offline-mode breakage.
3. **Historical trend**: If ApeGuard runs on the same repo multiple times, trend data across scans would be valuable — scores that INCREASE over time signal regression.
4. **False sense of precision**: A single 0-1 score can be misleading. Always show dimension breakdown so users can understand WHY a finding scored what it did.

## Relationship to Options A-C

| Option | Depends on | Integration |
|--------|-----------|-------------|
| A (Signal + Resume + JSONL) | — | Provides the data stream that feeds scoring |
| B (Skill Two-Layer) | — | Independent (agent infrastructure) |
| C (Grade Engine) | A | Provides AI verification → feeds `grade` dimension |
| **D (Scoring Pipeline)** | A, C (optional) | Consumes all other outputs into unified scores |

Without Option C, the `grade` dimension defaults to neutral (0.5) and scoring still works — just with less signal.

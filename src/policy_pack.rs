//! Built-in container/Kubernetes hardening policy pack.
//!
//! The pack encodes CIS Docker Benchmark and NSA/CISA Kubernetes Hardening Guide
//! controls against findings that the Checkov and Trivy scanners have already
//! produced (`CKV_DOCKER_*`, `CKV_K8S_*`, `AVD-KSV-*`, `DS002`, `DS026`). It does
//! not scan anything itself: it only re-prioritises and annotates existing
//! findings, which keeps every action traceable back to a scanner result.
//!
//! Composition: the pack and any user policy both define
//! `data.apeguard.policy_actions`, so they cannot be evaluated together. The pack
//! is therefore a *fallback* — it is only used when the configured policy
//! directory yields no `.rego` files. A user pack always wins.

/// Built-in policies as `(name, rego_source)` pairs, matching the shape returned
/// by [`crate::policy::load_policy_files`].
pub(crate) const DEFAULT_POLICIES: &[(&str, &str)] = &[(
    "builtin/hardening.rego",
    include_str!("../policies/hardening.rego"),
)];

/// Built-in policy pack as owned `(name, source)` pairs.
pub(crate) fn default_policies() -> Vec<(String, String)> {
    DEFAULT_POLICIES
        .iter()
        .map(|(name, source)| ((*name).to_string(), (*source).to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_pack_is_non_empty_and_named() {
        let policies = default_policies();
        assert_eq!(policies.len(), DEFAULT_POLICIES.len());
        assert_eq!(policies[0].0, "builtin/hardening.rego");
        assert!(policies[0].1.contains("package apeguard"));
        assert!(policies[0].1.contains("policy_actions"));
    }
}

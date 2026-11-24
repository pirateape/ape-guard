# ApeGuard built-in container / Kubernetes hardening policy pack.
#
# Standards encoded:
#   CIS Docker Benchmark v1.6                (host config, container runtime)
#   NSA/CISA Kubernetes Hardening Guide v1.2 (pod security, RBAC, network)
#
# Rules only act on findings produced by an IaC / container-config scanner
# (Checkov `CKV_DOCKER_*` / `CKV_K8S_*`, Trivy config `DS002` / `DS026`), so a
# dependency or secret finding that happens to share a keyword is untouched.
# The built-in pack is used ONLY when the user's policy dir has no .rego files.
#
# Rego v1 syntax (regorus 0.12): `contains` + `if` are mandatory. Action objects
# are written as head terms instead of a shared `entry` variable, because
# regorus rejects the same local variable name across several rules of one
# partial-set ("duplicated definition of local variable") once two rules fire.
package apeguard

# ---------------------------------------------------------------------------
# Tier 1: direct host compromise / cluster takeover -> Critical
# ---------------------------------------------------------------------------
policy_actions contains {
	"finding_id": f.id,
	"action": "escalate",
	"severity": "Critical",
	"reason": sprintf("container/Kubernetes finding matches NSA/CISA or CIS critical control (%s)", [kw]),
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in critical_keywords
	contains(lower(f.title), kw)
}

policy_actions contains {
	"finding_id": f.id,
	"action": "tag",
	"tags": ["hardening", "container-escape"],
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in critical_keywords
	contains(lower(f.title), kw)
}

# ---------------------------------------------------------------------------
# Tier 2: privilege / isolation weakeners -> High
# ---------------------------------------------------------------------------
policy_actions contains {
	"finding_id": f.id,
	"action": "escalate",
	"severity": "High",
	"reason": sprintf("pod/container isolation control missing (%s) - NSA/CISA K8s Hardening Guide", [kw]),
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in high_keywords
	contains(lower(f.title), kw)
}

policy_actions contains {
	"finding_id": f.id,
	"action": "tag",
	"tags": ["hardening", "pod-security"],
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in high_keywords
	contains(lower(f.title), kw)
}

# ---------------------------------------------------------------------------
# Tier 3: hygiene controls, cheap fix, rarely urgent alone -> Medium
# ---------------------------------------------------------------------------
policy_actions contains {
	"finding_id": f.id,
	"action": "escalate",
	"severity": "Medium",
	"reason": sprintf("container hygiene control (%s) - CIS Docker Benchmark", [kw]),
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in medium_keywords
	contains(lower(f.title), kw)
}

policy_actions contains {
	"finding_id": f.id,
	"action": "tag",
	"tags": ["hardening", "container-hygiene"],
} if {
	some i
	f := input.findings[i]
	is_container_config(f)
	some kw in medium_keywords
	contains(lower(f.title), kw)
}

# ---------------------------------------------------------------------------
# Selectors
# ---------------------------------------------------------------------------

# True when the finding came from an IaC / container-config scanner.
is_container_config(f) if {
	some prefix in container_prefixes
	startswith(f.rule_id, prefix)
}

container_prefixes := ["CKV_DOCKER_", "CKV_K8S_", "AVD-KSV-", "DS002", "DS026"]

critical_keywords := [
	"privileged",
	"hostpath",
	"host path",
	"cluster-admin",
	"cluster admin",
	"host network",
	"hostnetwork",
	"host pid",
	"hostpid",
	"hostipc",
	"host ipc",
]

high_keywords := [
	"runs root",
	"run root",
	"runasnonroot",
	"privilege escalation",
	"allowprivilegeescalation",
	"capabilit",
	"seccomp",
	"apparmor",
]

medium_keywords := [
	"read-only root",
	"readonlyrootfs",
	"read only root",
	"latest",
	"service account token",
	"automount",
	"network polic",
	"default namespace",
	"resource limit",
	"resource limits",
]

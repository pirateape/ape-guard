package apeguard

# Block all Critical-severity Gitleaks findings
policy_actions contains entry if {
    f := input.findings[_]
    f.severity == "Critical"
    f.scanner == "Gitleaks"

    entry := {
        "finding_id": f.id,
        "action": "block",
        "reason": "Critical secret detected — must be reviewed before proceeding",
        "message": null,
        "severity": null,
        "tags": ["blocked-by-policy"],
    }
}

# Escalate Medium Gitleaks secrets to High
policy_actions contains entry if {
    f := input.findings[_]
    f.severity == "Medium"
    f.scanner == "Gitleaks"

    entry := {
        "finding_id": f.id,
        "action": "escalate",
        "severity": "High",
        "message": null,
        "reason": "Secrets escalated by policy — medium severity in codebase",
        "tags": ["policy-escalated"],
    }
}

# Tag all Semgrep findings with "code-quality"
policy_actions contains entry if {
    f := input.findings[_]
    f.scanner == "Semgrep"

    entry := {
        "finding_id": f.id,
        "action": "tag",
        "tags": ["code-quality"],
        "message": null,
        "reason": "Semgrep findings tagged for code quality tracking",
        "severity": null,
    }
}

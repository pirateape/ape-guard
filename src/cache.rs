// ApeGuard Cache Module
// SQLite-based cache for incremental scanning.
// Stores file content hashes and their last scan result.
use crate::find::CanonicalFinding;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::{Path, PathBuf};

pub struct ScanCache {
    conn: Connection,
    enabled: bool,
    db_path: PathBuf,
}

pub struct RecordScanInput<'a> {
    pub scan_id: &'a str,
    pub target: &'a str,
    pub started_at: &'a str,
    pub completed_at: &'a str,
    pub total_findings: u32,
    pub scanners_used: &'a [String],
    pub findings: &'a [CanonicalFinding],
}

impl ScanCache {
    /// Open or create the SQLite cache database
    pub fn open(cache_dir: &Path) -> anyhow::Result<Self> {
        std::fs::create_dir_all(cache_dir)?;
        let db_path = cache_dir.join("apeguard.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS scan_history (
                scan_id TEXT PRIMARY KEY,
                target TEXT NOT NULL,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                total_findings INTEGER DEFAULT 0,
                scanners_used TEXT,
                findings_snapshot TEXT
            );",
        )?;

        // Migration: add findings_snapshot column if upgrading from older schema
        let _ = conn.execute(
            "ALTER TABLE scan_history ADD COLUMN findings_snapshot TEXT",
            [],
        );

        Ok(ScanCache {
            conn,
            enabled: true,
            db_path,
        })
    }

    /// Create a disabled cache (no-op)
    pub fn disabled() -> Self {
        ScanCache {
            conn: Connection::open_in_memory().unwrap(),
            enabled: false,
            db_path: PathBuf::from(":memory:"),
        }
    }

    /// Get findings snapshot for a specific scan
    pub fn get_scan_findings(
        &self,
        scan_id: &str,
    ) -> anyhow::Result<Option<Vec<CanonicalFinding>>> {
        if !self.enabled {
            return Ok(None);
        }

        let mut stmt = self
            .conn
            .prepare("SELECT findings_snapshot FROM scan_history WHERE scan_id = ?1")?;

        let result: Option<String> = stmt.query_row(params![scan_id], |row| row.get(0)).ok();

        match result {
            Some(json) => {
                let findings: Vec<CanonicalFinding> = serde_json::from_str(&json)?;
                Ok(Some(findings))
            }
            None => Ok(None),
        }
    }

    /// Get findings for the most recent scan
    pub fn get_latest_scan_findings(
        &self,
    ) -> anyhow::Result<Option<(String, Vec<CanonicalFinding>)>> {
        if !self.enabled {
            return Ok(None);
        }

        let mut stmt = self.conn.prepare(
            "SELECT scan_id, findings_snapshot FROM scan_history ORDER BY started_at DESC LIMIT 1",
        )?;

        let result: Option<(String, String)> = stmt
            .query_row([], |row| Ok((row.get(0)?, row.get(1)?)))
            .ok();

        match result {
            Some((scan_id, json)) => {
                let findings: Vec<CanonicalFinding> = serde_json::from_str(&json)?;
                Ok(Some((scan_id, findings)))
            }
            None => Ok(None),
        }
    }

    /// Get the latest scan record metadata (without findings)
    pub fn get_latest_scan_record(&self) -> anyhow::Result<Option<ScanRecord>> {
        if !self.enabled {
            return Ok(None);
        }

        let mut stmt = self.conn.prepare(
            "SELECT scan_id, target, started_at, completed_at, total_findings, scanners_used
             FROM scan_history ORDER BY started_at DESC LIMIT 1",
        )?;

        let result = stmt
            .query_row([], |row| {
                Ok(ScanRecord {
                    scan_id: row.get(0)?,
                    target: row.get(1)?,
                    started_at: row.get(2)?,
                    completed_at: row.get(3)?,
                    total_findings: row.get(4)?,
                    scanners_used: row.get(5)?,
                })
            })
            .ok();

        Ok(result)
    }

    /// Get cache statistics: file count, scan count, database size
    pub fn stats(&self) -> anyhow::Result<CacheStats> {
        let scan_count: u64 = if self.enabled {
            self.conn
                .query_row("SELECT COUNT(*) FROM scan_history", [], |row| row.get(0))
                .unwrap_or(0)
        } else {
            0
        };

        let total_findings: u64 = if self.enabled {
            self.conn
                .query_row(
                    "SELECT COALESCE(SUM(total_findings), 0) FROM scan_history",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0)
        } else {
            0
        };

        let db_size = std::fs::metadata(&self.db_path)
            .map(|m| m.len())
            .unwrap_or(0);

        Ok(CacheStats {
            scan_count,
            total_findings,
            database_size_bytes: db_size,
            enabled: self.enabled,
        })
    }

    /// Prune old cache entries, keeping only the N most recent scans
    pub fn prune(&self, keep: u32) -> anyhow::Result<u64> {
        if !self.enabled {
            return Ok(0);
        }

        // Find the cutoff scan_id — keep the N most recent
        let cutoff: Option<String> = self
            .conn
            .query_row(
                "SELECT scan_id FROM scan_history ORDER BY started_at DESC LIMIT 1 OFFSET ?1",
                params![keep],
                |row| row.get(0),
            )
            .ok();

        let removed = if let Some(cutoff_id) = cutoff {
            // Remove old scan records
            self.conn.execute(
                "DELETE FROM scan_history WHERE started_at < (SELECT started_at FROM scan_history WHERE scan_id = ?1)",
                params![cutoff_id],
            )?;

            // Remove cache entries not referenced by recent scans (keep all for now — hard to
            // determine which files were scanned in which sessions without a join table)
            // For simplicity, just count what we removed from scan_history
            self.conn
                .query_row("SELECT changes()", [], |row| row.get::<_, u64>(0))?
        } else {
            // Fewer scans than keep threshold, nothing to prune
            0
        };

        // VACUUM to reclaim disk space
        if removed > 0 {
            let _ = self.conn.execute_batch("VACUUM;");
        }

        Ok(removed)
    }

    /// Enforce TTL by removing entries older than `ttl_hours`.
    pub fn enforce_ttl(&self, ttl_hours: u32) -> anyhow::Result<u64> {
        if !self.enabled || ttl_hours == 0 {
            return Ok(0);
        }

        let cutoff = chrono::Utc::now() - chrono::Duration::hours(ttl_hours as i64);

        self.conn.execute(
            "DELETE FROM scan_history WHERE started_at < ?1",
            params![cutoff.to_rfc3339()],
        )?;
        let removed_history: u64 = self
            .conn
            .query_row("SELECT changes()", [], |row| row.get(0))?;

        let removed = removed_history;

        if removed > 0 {
            let _ = self.conn.execute_batch("VACUUM;");
        }

        Ok(removed)
    }

    /// Record a scan session in history with findings snapshot
    pub fn record_scan(&self, input: RecordScanInput<'_>) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let scanners = input.scanners_used.join(",");
        let findings_json = serde_json::to_string(input.findings)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO scan_history (scan_id, target, started_at, completed_at, total_findings, scanners_used, findings_snapshot)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                input.scan_id,
                input.target,
                input.started_at,
                input.completed_at,
                input.total_findings,
                scanners,
                findings_json
            ],
        )?;

        Ok(())
    }

    /// Get the last N scan summaries
    pub fn recent_scans(&self, limit: u32) -> anyhow::Result<Vec<ScanRecord>> {
        if !self.enabled {
            return Ok(vec![]);
        }

        let mut stmt = self.conn.prepare(
            "SELECT scan_id, target, started_at, completed_at, total_findings, scanners_used
             FROM scan_history ORDER BY started_at DESC LIMIT ?1",
        )?;

        let records = stmt
            .query_map(params![limit], |row| {
                Ok(ScanRecord {
                    scan_id: row.get(0)?,
                    target: row.get(1)?,
                    started_at: row.get(2)?,
                    completed_at: row.get(3)?,
                    total_findings: row.get(4)?,
                    scanners_used: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Public struct — fields used externally
pub struct ScanRecord {
    pub scan_id: String,
    pub target: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_findings: u32,
    pub scanners_used: String,
}

/// Statistics about the scan cache
#[derive(Debug, Clone, Serialize)]
pub struct CacheStats {
    pub scan_count: u64,
    pub total_findings: u64,
    pub database_size_bytes: u64,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};

    fn sample_finding() -> CanonicalFinding {
        CanonicalFinding {
            id: "F-1".to_string(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "generic-api-key".to_string(),
            severity: Severity::High,
            confidence: Confidence::Certain,
            title: "Secret found".to_string(),
            description: "Hardcoded key".to_string(),
            location: FindingLocation {
                file: PathBuf::from(".env"),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: Some("API_KEY=...".to_string()),
            },
            cwe: Some("CWE-798".to_string()),
            cvss: Some(7.5),
            remediation: Some("Rotate key".to_string()),
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
        }
    }

    #[test]
    fn test_record_scan_persists_timestamps() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = ScanCache::open(tmp.path()).unwrap();

        let findings = vec![sample_finding()];
        let started = "2026-01-01T00:00:00Z";
        let completed = "2026-01-01T00:05:00Z";
        cache
            .record_scan(RecordScanInput {
                scan_id: "scan-1",
                target: ".",
                started_at: started,
                completed_at: completed,
                total_findings: findings.len() as u32,
                scanners_used: &["gitleaks".to_string()],
                findings: &findings,
            })
            .unwrap();

        let row: (String, String) = cache
            .conn
            .query_row(
                "SELECT started_at, completed_at FROM scan_history WHERE scan_id = ?1",
                params!["scan-1"],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();

        assert_eq!(row.0, started);
        assert_eq!(row.1, completed);
    }

    #[test]
    fn test_enforce_ttl_prunes_old_scan_history() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = ScanCache::open(tmp.path()).unwrap();

        // Insert very old record directly
        cache
            .conn
            .execute(
                "INSERT INTO scan_history (scan_id, target, started_at, completed_at, total_findings, scanners_used, findings_snapshot)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    "old-scan",
                    ".",
                    "2000-01-01T00:00:00Z",
                    "2000-01-01T00:01:00Z",
                    0u32,
                    "gitleaks",
                    "[]"
                ],
            )
            .unwrap();

        let removed = cache.enforce_ttl(24).unwrap();
        assert!(removed >= 1);

        let old_exists: Option<String> = cache
            .conn
            .query_row(
                "SELECT scan_id FROM scan_history WHERE scan_id = ?1",
                params!["old-scan"],
                |r| r.get(0),
            )
            .ok();
        assert!(old_exists.is_none());
    }
}

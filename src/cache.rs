// ApeGuard Cache Module
// SQLite-based cache for incremental scanning.
// Stores file content hashes and their last scan result.
use crate::find::CanonicalFinding;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct ScanCache {
    conn: Connection,
    enabled: bool,
}

impl ScanCache {
    /// Open or create the SQLite cache database
    pub fn open(cache_dir: &Path) -> anyhow::Result<Self> {
        std::fs::create_dir_all(cache_dir)?;
        let db_path = cache_dir.join("apeguard.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cache (
                file_hash TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                findings TEXT NOT NULL,
                scanned_at TEXT NOT NULL,
                scanner_version TEXT
            );
            CREATE TABLE IF NOT EXISTS scan_history (
                scan_id TEXT PRIMARY KEY,
                target TEXT NOT NULL,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                total_findings INTEGER DEFAULT 0,
                scanners_used TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_cache_path ON cache(file_path);",
        )?;

        Ok(ScanCache {
            conn,
            enabled: true,
        })
    }

    /// Create a disabled cache (no-op)
    pub fn disabled() -> Self {
        ScanCache {
            conn: Connection::open_in_memory().unwrap(),
            enabled: false,
        }
    }

    /// Compute SHA-256 hash of file contents
    pub fn hash_file(path: &Path) -> anyhow::Result<String> {
        let contents = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Check if a file has been scanned with the same content hash
    pub fn get_cached(&self, file_hash: &str) -> anyhow::Result<Option<Vec<CanonicalFinding>>> {
        if !self.enabled {
            return Ok(None);
        }

        let mut stmt = self
            .conn
            .prepare("SELECT findings FROM cache WHERE file_hash = ?1")?;

        let result: Option<String> = stmt
            .query_row(params![file_hash], |row| row.get(0))
            .ok();

        match result {
            Some(json) => {
                let findings: Vec<CanonicalFinding> = serde_json::from_str(&json)?;
                Ok(Some(findings))
            }
            None => Ok(None),
        }
    }

    /// Store scan results for a file hash
    pub fn store(
        &self,
        file_hash: &str,
        file_path: &str,
        findings: &[CanonicalFinding],
        scanner_version: Option<&str>,
    ) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let json = serde_json::to_string(findings)?;
        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO cache (file_hash, file_path, findings, scanned_at, scanner_version)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![file_hash, file_path, json, now, scanner_version],
        )?;

        Ok(())
    }

    /// Record a scan session in history
    pub fn record_scan(
        &self,
        scan_id: &str,
        target: &str,
        total_findings: u32,
        scanners_used: &[String],
    ) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let now = chrono::Utc::now().to_rfc3339();
        let scanners = scanners_used.join(",");

        self.conn.execute(
            "INSERT INTO scan_history (scan_id, target, started_at, completed_at, total_findings, scanners_used)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![scan_id, target, now, now, total_findings, scanners],
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
pub struct ScanRecord {
    pub scan_id: String,
    pub target: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_findings: u32,
    pub scanners_used: String,
}

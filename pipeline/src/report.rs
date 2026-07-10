use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs, io};

/// PROJECT.md §9: every run emits a build report. This is the
/// in-process version; persisting it to disk/the run manifest is a
/// follow-up once the Snapshot stage has a real place to write it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub struct StageRecord {
    pub name: &'static str,
    pub status: StageStatus,
    pub duration_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct BuildReport {
    pub run_id: String,
    pub started_at: DateTime<Utc>,
    #[serde(skip)]
    started_at_instant: Instant,
    pub stages: Vec<StageRecord>,
}

impl BuildReport {
    pub fn new(run_id: String) -> Self {
        Self {
            run_id,
            started_at: Utc::now(),
            started_at_instant: Instant::now(),
            stages: Vec::new(),
        }
    }

    pub fn record(&mut self, name: &'static str, status: StageStatus, duration: Duration) {
        self.stages.push(StageRecord {
            name,
            status,
            duration_ms: duration.as_millis(),
        });
    }

    /// Failed if any stage failed, otherwise Ok. Degraded-vs-Ok
    /// distinctions (PROJECT.md §9) need real per-stage semantics
    /// (e.g. "validation passed with warnings") that don't exist yet.
    pub fn overall_status(&self) -> StageStatus {
        if self.stages.iter().any(|s| s.status == StageStatus::Failed) {
            StageStatus::Failed
        } else {
            StageStatus::Ok
        }
    }

    pub fn total_duration(&self) -> Duration {
        self.started_at_instant.elapsed()
    }

    pub fn write_json(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, format!("{json}\n"))
    }

    pub fn print_summary(&self) {
        println!("\nBuild report — run {}", self.run_id);
        for s in &self.stages {
            println!("  {:<10} {:?}  ({}ms)", s.name, s.status, s.duration_ms);
        }
        println!(
            "Status: {:?}  Total: {:.2?}",
            self.overall_status(),
            self.total_duration()
        );
    }
}

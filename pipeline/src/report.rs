use std::time::{Duration, Instant};

/// PROJECT.md §9: every run emits a build report. This is the
/// in-process version; persisting it to disk/the run manifest is a
/// follow-up once the Snapshot stage has a real place to write it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone)]
pub struct StageRecord {
    pub name: &'static str,
    pub status: StageStatus,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct BuildReport {
    pub run_id: String,
    started_at: Instant,
    pub stages: Vec<StageRecord>,
}

impl BuildReport {
    pub fn new(run_id: String) -> Self {
        Self {
            run_id,
            started_at: Instant::now(),
            stages: Vec::new(),
        }
    }

    pub fn record(&mut self, name: &'static str, status: StageStatus, duration: Duration) {
        self.stages.push(StageRecord { name, status, duration });
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
        self.started_at.elapsed()
    }

    pub fn print_summary(&self) {
        println!("\nBuild report — run {}", self.run_id);
        for s in &self.stages {
            println!("  {:<10} {:?}  ({:.2?})", s.name, s.status, s.duration);
        }
        println!(
            "Status: {:?}  Total: {:.2?}",
            self.overall_status(),
            self.total_duration()
        );
    }
}

use std::path::PathBuf;
use std::{fs, io};

/// Shared state threaded through every stage. Intentionally thin right
/// now -- it grows as each stage gains real behavior (e.g. the raw
/// clone output path after Clone, the dataset handle after Extract).
pub struct PipelineContext {
    /// Root directory for this project's data (snapshots/, current, etc.).
    /// See PROJECT.md §4 for the snapshot layout.
    pub data_root: PathBuf,

    /// Timestamp identifying the snapshot this run is building, e.g.
    /// "2026-07-09T12-00Z" (PROJECT.md §4). Assigned once, at pipeline
    /// start, by the scheduler.
    pub run_id: String,
}

impl PipelineContext {
    pub fn new(data_root: PathBuf, run_id: String) -> Self {
        Self { data_root, run_id }
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.data_root.join("runs")
    }

    pub fn run_dir(&self) -> PathBuf {
        self.runs_dir().join(&self.run_id)
    }

    pub fn stage_dir(&self, stage: &str) -> PathBuf {
        self.run_dir().join(stage)
    }

    pub fn snapshots_dir(&self) -> PathBuf {
        self.data_root.join("snapshots")
    }

    pub fn snapshot_dir(&self) -> PathBuf {
        self.snapshots_dir().join(&self.run_id)
    }

    pub fn reports_dir(&self) -> PathBuf {
        self.data_root.join("reports")
    }

    pub fn build_report_path(&self) -> PathBuf {
        self.reports_dir().join(format!("{}.json", self.run_id))
    }

    pub fn current_pointer_path(&self) -> PathBuf {
        self.data_root.join("current.json")
    }

    pub fn ensure_layout(&self) -> io::Result<()> {
        fs::create_dir_all(self.run_dir())?;
        fs::create_dir_all(self.snapshots_dir())?;
        fs::create_dir_all(self.reports_dir())
    }

    pub fn ensure_stage_dir(&self, stage: &str) -> io::Result<PathBuf> {
        let path = self.stage_dir(stage);
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn write_stage_manifest(
        &self,
        stage: &str,
        body: &serde_json::Value,
    ) -> io::Result<PathBuf> {
        let dir = self.ensure_stage_dir(stage)?;
        let path = dir.join("artifact.json");
        let json = serde_json::to_string_pretty(body)?;
        fs::write(&path, format!("{json}\n"))?;
        Ok(path)
    }
}

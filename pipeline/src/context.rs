use std::path::PathBuf;

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
}

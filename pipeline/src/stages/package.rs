use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Packages the snapshot + hash manifest into torrents, and separately
/// assembles the periodic research-dataset bundle. See PROJECT.md §5, §8.
pub struct PackageStage;

impl Stage for PackageStage {
    fn name(&self) -> &'static str {
        "package"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: build the per-snapshot torrent(s); on the research
        // dataset cadence (still open, PROJECT.md §10), also build the
        // research-dataset bundle.
        println!("  [package] stub -- would build torrents/research bundle here");
        Ok(())
    }
}

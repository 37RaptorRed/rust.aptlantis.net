use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Packages the snapshot + hash manifest into torrents, and separately
/// assembles the periodic research-dataset bundle. See PROJECT.md §5, §8.
pub struct PackageStage;

impl Stage for PackageStage {
    fn name(&self) -> &'static str {
        "package"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: build the per-snapshot torrent(s); on the research
        // dataset cadence (still open, PROJECT.md §10), also build the
        // research-dataset bundle.
        let hash = ctx.stage_dir("hash").join("artifact.json");
        anyhow::ensure!(
            hash.exists(),
            "hash artifact is missing at {}",
            hash.display()
        );
        let packages_dir = ctx.snapshot_dir().join("packages");
        std::fs::create_dir_all(packages_dir.join("torrents"))?;
        std::fs::create_dir_all(packages_dir.join("research-dataset"))?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "package",
                "status": "contract_only",
                "input": hash,
                "packages_dir": packages_dir,
                "open_decisions": ["torrent_granularity", "research_dataset_cadence"]
            }),
        )?;
        println!("  [package] package directory contract written");
        Ok(())
    }
}

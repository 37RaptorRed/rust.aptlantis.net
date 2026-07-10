use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Builds/deploys the Astro site from this snapshot's dataset and
/// seeds the packaged torrents. See PROJECT.md §5.
pub struct PublishStage;

impl Stage for PublishStage {
    fn name(&self) -> &'static str {
        "publish"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: trigger the Astro build and torrent seeding.
        let package = ctx.stage_dir("package").join("artifact.json");
        anyhow::ensure!(
            package.exists(),
            "package artifact is missing at {}",
            package.display()
        );
        let site_dir = ctx.snapshot_dir().join("site");
        std::fs::create_dir_all(&site_dir)?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "publish",
                "status": "contract_only",
                "input": package,
                "site_dir": site_dir,
                "publish_targets": ["astro_static_site", "torrent_seed"]
            }),
        )?;
        println!("  [publish] publish contract written; Astro/torrent seed pending");
        Ok(())
    }
}

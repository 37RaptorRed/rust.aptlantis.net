use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Builds/deploys the Astro site from this snapshot's dataset and
/// seeds the packaged torrents. See PROJECT.md §5.
pub struct PublishStage;

impl Stage for PublishStage {
    fn name(&self) -> &'static str {
        "publish"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: trigger the Astro build and torrent seeding.
        println!("  [publish] stub -- would build/deploy the Astro site and seed torrents here");
        Ok(())
    }
}

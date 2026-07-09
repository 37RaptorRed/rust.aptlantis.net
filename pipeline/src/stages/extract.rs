use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Parses promoted `crate.json` files into the structured dataset
/// (versions, yank flags, dependency edges). See PROJECT.md §5, §6.
pub struct ExtractStage;

impl Stage for ExtractStage {
    fn name(&self) -> &'static str {
        "extract"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: parse crate.json files into the structured dataset.
        // Dataset schema itself is a separate, not-yet-defined chunk.
        println!("  [extract] stub -- would parse crate.json into the dataset here");
        Ok(())
    }
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Delegates to CloneCratesio (D:\CTS\CloneCratesio) to pull the
/// crates.io index + .crate files, yanked-aware. See PROJECT.md §5.
pub struct CloneStage;

impl Stage for CloneStage {
    fn name(&self) -> &'static str {
        "clone"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: invoke CloneCratesio as a subprocess/job and record the
        // raw output location in the context for the Validate stage.
        println!("  [clone] stub -- would invoke CloneCratesio here");
        Ok(())
    }
}

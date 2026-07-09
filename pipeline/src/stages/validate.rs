use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Checks the Clone stage's output (record counts, checksum
/// spot-checks) before anything downstream is allowed to promote it.
/// See PROJECT.md §5.
pub struct ValidateStage;

impl Stage for ValidateStage {
    fn name(&self) -> &'static str {
        "validate"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: verify counts/checksums against the Clone stage's
        // output; return Err(..) on failure so the pipeline stops here
        // instead of promoting bad data.
        println!("  [validate] stub -- would check counts/checksums here");
        Ok(())
    }
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Marks validated raw output as the current-candidate snapshot. Last
/// point before a snapshot is considered real. See PROJECT.md §5.
pub struct PromoteStage;

impl Stage for PromoteStage {
    fn name(&self) -> &'static str {
        "promote"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: mark the validated raw output as current-candidate.
        println!("  [promote] stub -- would mark current-candidate here");
        Ok(())
    }
}

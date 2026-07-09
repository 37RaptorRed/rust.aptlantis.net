use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Builds the time-series and analytics catalog (PROJECT.md §7) from
/// the structured dataset plus prior snapshots.
pub struct AnalyzeStage;

impl Stage for AnalyzeStage {
    fn name(&self) -> &'static str {
        "analyze"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: compute the §7 metric catalog against this dataset and
        // the prior snapshot's time-series.
        println!("  [analyze] stub -- would compute the analytics catalog here");
        Ok(())
    }
}

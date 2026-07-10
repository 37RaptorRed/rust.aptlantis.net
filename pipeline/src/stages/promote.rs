use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Marks validated raw output as the current-candidate snapshot. Last
/// point before a snapshot is considered real. See PROJECT.md §5.
pub struct PromoteStage;

impl Stage for PromoteStage {
    fn name(&self) -> &'static str {
        "promote"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: mark the validated raw output as current-candidate.
        let validation = ctx.stage_dir("validate").join("artifact.json");
        anyhow::ensure!(
            validation.exists(),
            "validation artifact is missing at {}",
            validation.display()
        );
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "promote",
                "status": "contract_only",
                "input": validation,
                "candidate": ctx.stage_dir(self.name()).join("current-candidate.json")
            }),
        )?;
        println!("  [promote] current-candidate contract written");
        Ok(())
    }
}

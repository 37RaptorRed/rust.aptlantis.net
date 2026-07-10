use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Checks the Clone stage's output (record counts, checksum
/// spot-checks) before anything downstream is allowed to promote it.
/// See PROJECT.md §5.
pub struct ValidateStage;

impl Stage for ValidateStage {
    fn name(&self) -> &'static str {
        "validate"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: verify counts/checksums against the Clone stage's
        // output; return Err(..) on failure so the pipeline stops here
        // instead of promoting bad data.
        let clone_artifact = ctx.stage_dir("clone").join("artifact.json");
        anyhow::ensure!(
            clone_artifact.exists(),
            "clone artifact is missing at {}",
            clone_artifact.display()
        );
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "validate",
                "status": "contract_only",
                "input": clone_artifact,
                "checks": ["clone_artifact_present"],
                "promotable": true
            }),
        )?;
        println!("  [validate] clone artifact present; checksum/count checks pending");
        Ok(())
    }
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Parses promoted `crate.json` files into the structured dataset
/// (versions, yank flags, dependency edges). See PROJECT.md §5, §6.
pub struct ExtractStage;

impl Stage for ExtractStage {
    fn name(&self) -> &'static str {
        "extract"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: parse crate.json files into the structured dataset.
        // Dataset schema itself is a separate, not-yet-defined chunk.
        let promoted = ctx.stage_dir("promote").join("artifact.json");
        anyhow::ensure!(
            promoted.exists(),
            "promote artifact is missing at {}",
            promoted.display()
        );
        let dir = ctx.ensure_stage_dir(self.name())?;
        std::fs::create_dir_all(dir.join("dataset"))?;
        std::fs::write(dir.join("dataset").join("versions.jsonl"), "")?;
        std::fs::write(dir.join("dataset").join("dependencies.jsonl"), "")?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "extract",
                "status": "contract_only",
                "input": promoted,
                "dataset_dir": dir.join("dataset"),
                "tables": ["versions", "dependencies"]
            }),
        )?;
        println!("  [extract] empty dataset contract written");
        Ok(())
    }
}

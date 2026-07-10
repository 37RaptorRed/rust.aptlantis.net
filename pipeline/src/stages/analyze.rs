use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Builds the time-series and analytics catalog (PROJECT.md §7) from
/// the structured dataset plus prior snapshots.
pub struct AnalyzeStage;

impl Stage for AnalyzeStage {
    fn name(&self) -> &'static str {
        "analyze"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: compute the §7 metric catalog against this dataset and
        // the prior snapshot's time-series.
        let extract = ctx.stage_dir("extract").join("artifact.json");
        anyhow::ensure!(
            extract.exists(),
            "extract artifact is missing at {}",
            extract.display()
        );
        let dir = ctx.ensure_stage_dir(self.name())?;
        std::fs::create_dir_all(dir.join("analytics"))?;
        std::fs::write(
            dir.join("analytics").join("metric-catalog.json"),
            "{\n  \"status\": \"contract_only\",\n  \"metrics\": []\n}\n",
        )?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "analyze",
                "status": "contract_only",
                "input": extract,
                "analytics_dir": dir.join("analytics")
            }),
        )?;
        println!("  [analyze] analytics catalog contract written");
        Ok(())
    }
}

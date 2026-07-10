use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Delegates to CloneCratesio (D:\CTS\CloneCratesio) to pull the
/// crates.io index + .crate files, yanked-aware. See PROJECT.md §5.
pub struct CloneStage;

impl Stage for CloneStage {
    fn name(&self) -> &'static str {
        "clone"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: invoke CloneCratesio as a subprocess/job and record the
        // raw output location in the context for the Validate stage.
        let dir = ctx.ensure_stage_dir(self.name())?;
        std::fs::create_dir_all(dir.join("raw-index"))?;
        std::fs::create_dir_all(dir.join("crate-files"))?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "clone",
                "status": "contract_only",
                "component": "D:\\CTS\\CloneCratesio",
                "raw_index_dir": dir.join("raw-index"),
                "crate_files_dir": dir.join("crate-files")
            }),
        )?;
        println!("  [clone] contract artifact written; CloneCratesio integration pending");
        Ok(())
    }
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;
use std::fs;

/// Freezes an immutable `snapshots/<run_id>/` directory and moves the
/// `current` pointer to it. See PROJECT.md §4.
pub struct SnapshotStage;

impl Stage for SnapshotStage {
    fn name(&self) -> &'static str {
        "snapshot"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        let analytics = ctx.stage_dir("analyze").join("artifact.json");
        anyhow::ensure!(
            analytics.exists(),
            "analyze artifact is missing at {}",
            analytics.display()
        );
        let snapshot_dir = ctx.snapshot_dir();
        fs::create_dir_all(snapshot_dir.join("raw"))?;
        fs::create_dir_all(snapshot_dir.join("dataset"))?;
        fs::create_dir_all(snapshot_dir.join("analytics"))?;
        let snapshot_manifest = json!({
            "run_id": ctx.run_id,
            "status": "contract_only",
            "source_run_dir": ctx.run_dir()
        });
        fs::write(
            snapshot_dir.join("snapshot.json"),
            format!("{}\n", serde_json::to_string_pretty(&snapshot_manifest)?),
        )?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "snapshot",
                "status": "contract_only",
                "snapshot_dir": snapshot_dir
            }),
        )?;

        let pointer = json!({
            "run_id": ctx.run_id,
            "snapshot_dir": ctx.snapshot_dir()
        });
        let tmp_pointer = ctx.data_root.join("current.json.tmp");
        fs::write(
            &tmp_pointer,
            format!("{}\n", serde_json::to_string_pretty(&pointer)?),
        )?;
        if ctx.current_pointer_path().exists() {
            fs::remove_file(ctx.current_pointer_path())?;
        }
        fs::rename(tmp_pointer, ctx.current_pointer_path())?;
        println!(
            "  [snapshot] froze contract snapshot {} under {}",
            ctx.run_id,
            ctx.data_root.display()
        );
        Ok(())
    }
}

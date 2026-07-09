use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Freezes an immutable `snapshots/<run_id>/` directory and moves the
/// `current` pointer to it. See PROJECT.md §4.
pub struct SnapshotStage;

impl Stage for SnapshotStage {
    fn name(&self) -> &'static str {
        "snapshot"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: write snapshots/<run_id>/ under ctx.data_root and only
        // then move the `current` pointer -- current must never point
        // at a partially-written snapshot.
        println!(
            "  [snapshot] stub -- would freeze snapshots/{} under {} here",
            ctx.run_id,
            ctx.data_root.display()
        );
        Ok(())
    }
}

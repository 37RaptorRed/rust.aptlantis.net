use crate::context::PipelineContext;

pub type StageResult = anyhow::Result<()>;

/// A single stage in the sync pipeline (PROJECT.md §5).
///
/// Each stage only reads artifacts produced by the stage(s) before it,
/// so a failure partway through the pipeline can be resumed from the
/// failed stage instead of restarting from Clone.
pub trait Stage {
    /// Short, stable name used in logs and the build report.
    fn name(&self) -> &'static str;

    /// Run this stage. Stages should be side-effecting only through
    /// `ctx` (e.g. writing into the snapshot directory) so the pipeline
    /// runner can record start/end/duration/status uniformly.
    fn run(&self, ctx: &mut PipelineContext) -> StageResult;
}

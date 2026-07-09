mod analyze;
mod clone;
mod extract;
mod hash;
mod package;
mod promote;
mod publish;
mod snapshot;
mod validate;

use crate::stage::Stage;

/// The pipeline's stage order (PROJECT.md §3, §5). This is the single
/// place that defines "what order do we run in" -- add new stages here.
pub fn all_stages() -> Vec<Box<dyn Stage>> {
    vec![
        Box::new(clone::CloneStage),
        Box::new(validate::ValidateStage),
        Box::new(promote::PromoteStage),
        Box::new(extract::ExtractStage),
        Box::new(analyze::AnalyzeStage),
        Box::new(snapshot::SnapshotStage),
        Box::new(hash::HashStage),
        Box::new(package::PackageStage),
        Box::new(publish::PublishStage),
    ]
}

mod context;
mod report;
mod scheduler;
mod stage;
mod stages;

use context::PipelineContext;
use report::{BuildReport, StageStatus};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    if !scheduler::should_run_now() {
        println!("Not due for a sync yet (target freshness: {}h).", scheduler::FRESHNESS_TARGET_HOURS);
        return Ok(());
    }

    let run_id = scheduler::next_run_id();
    println!("Starting pipeline run {run_id}");

    let mut ctx = PipelineContext::new(PathBuf::from("./data"), run_id.clone());
    let mut report = BuildReport::new(run_id);

    for stage in stages::all_stages() {
        let started = Instant::now();
        println!("-> {}", stage.name());

        let result = stage.run(&mut ctx);
        let status = match &result {
            Ok(()) => StageStatus::Ok,
            Err(e) => {
                eprintln!("   stage '{}' failed: {e}", stage.name());
                StageStatus::Failed
            }
        };
        report.record(stage.name(), status, started.elapsed());

        // Stop on first failure -- see PROJECT.md §5 on recoverability:
        // a later re-run should resume from the failed stage, not redo
        // everything from Clone. Resume logic itself isn't built yet.
        if status == StageStatus::Failed {
            break;
        }
    }

    report.print_summary();
    Ok(())
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};
use serde_json::json;

/// Delegates to ArchiveHasher (D:\CTS\ArchiveHasher) to hash and sign
/// the frozen snapshot per the AAMHS standard
/// (D:\.library\aptlantis_core\AAMHS). See PROJECT.md §5.
pub struct HashStage;

impl Stage for HashStage {
    fn name(&self) -> &'static str {
        "hash"
    }

    fn run(&self, ctx: &mut PipelineContext) -> StageResult {
        // TODO: invoke ArchiveHasher against the frozen snapshot and
        // store its hash/signature manifest alongside it.
        let snapshot = ctx.snapshot_dir().join("snapshot.json");
        anyhow::ensure!(
            snapshot.exists(),
            "snapshot manifest is missing at {}",
            snapshot.display()
        );
        let integrity_dir = ctx.snapshot_dir().join("integrity");
        std::fs::create_dir_all(&integrity_dir)?;
        let placeholder = json!({
            "status": "contract_only",
            "component": "D:\\CTS\\ArchiveHasher",
            "standard": "AAMHS"
        });
        std::fs::write(
            integrity_dir.join("hash-manifest.placeholder.json"),
            format!("{}\n", serde_json::to_string_pretty(&placeholder)?),
        )?;
        ctx.write_stage_manifest(
            self.name(),
            &json!({
                "stage": "hash",
                "status": "contract_only",
                "component": "D:\\CTS\\ArchiveHasher",
                "standard": "AAMHS",
                "integrity_dir": integrity_dir
            }),
        )?;
        println!("  [hash] ArchiveHasher contract artifact written; invocation pending");
        Ok(())
    }
}

use crate::context::PipelineContext;
use crate::stage::{Stage, StageResult};

/// Delegates to ArchiveHasher (D:\CTS\ArchiveHasher) to hash and sign
/// the frozen snapshot per the AAMHS standard
/// (D:\.library\aptlantis_core\AAMHS). See PROJECT.md §5.
pub struct HashStage;

impl Stage for HashStage {
    fn name(&self) -> &'static str {
        "hash"
    }

    fn run(&self, _ctx: &mut PipelineContext) -> StageResult {
        // TODO: invoke ArchiveHasher against the frozen snapshot and
        // store its hash/signature manifest alongside it.
        println!("  [hash] stub -- would invoke ArchiveHasher (per AAMHS) here");
        Ok(())
    }
}

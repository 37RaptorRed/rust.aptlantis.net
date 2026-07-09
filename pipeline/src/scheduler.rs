use chrono::Utc;

/// Target freshness from PROJECT.md §12: never more than ~12h stale.
pub const FRESHNESS_TARGET_HOURS: i64 = 12;

/// Decides whether a run should start now.
///
/// Stub: always returns true. The real version needs to read the last
/// completed run's timestamp (from the build-report history, PROJECT.md
/// §9) and compare it against FRESHNESS_TARGET_HOURS -- that's what lets
/// this replace cron instead of just being cron-in-Rust.
pub fn should_run_now() -> bool {
    true
}

/// Snapshot id format from PROJECT.md §4, e.g. "2026-07-09T12-00Z".
pub fn next_run_id() -> String {
    Utc::now().format("%Y-%m-%dT%H-%MZ").to_string()
}

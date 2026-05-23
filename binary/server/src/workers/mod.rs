//! Worker modes for the [`server`](crate) binary.
//!
//! Each worker mode is a grouping of one or more core modules that will be
//! co-hosted in a single process. A real deployment runs this binary several
//! times, each invocation selecting a different mode via subcommand, so that
//! the cluster as a whole covers every module while keeping process roles
//! coarse and operator-friendly.
//!
//! The exact set of modes and which modules each mode hosts is **not yet
//! finalized**.

//! Convergence engine — state machine, persistence, WAL, checklist, routing.

pub mod build_freshness;
pub mod checklist;
pub mod convergence;
pub mod form_detect;
pub mod index;
pub mod persistence;
pub mod plan_status;
pub mod plan_validator;
pub mod priority;
pub mod router;
pub mod state;
pub mod test_runner;
pub mod toolchain;
pub mod wal;

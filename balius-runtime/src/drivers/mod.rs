//! Control-flow components on top of the Runtime.
//!
//! Drivers are responsible for implementing the control-flows that
//! handle external interactions (e.g. handling requests, syncing from the
//! blockchain, etc) and funnels them into the Runtime.
//!
//! Each of these drivers has a way to trigger a forever-loop that should be
//! spawn as an independent tokio task running on the background.

pub mod chainsync;
pub mod jsonrpc;

use crate::*;

/// Configuration parameters for server management.
///
/// Holds all necessary configuration values for server operation.
#[derive(Clone)]
pub struct ServerManagerConfig {
    /// Path to the PID file for process tracking.
    pub(crate) pid_file: String,
    /// An asynchronous function to be called before stopping the server.
    pub(crate) before_stop_hook: Hook,
    /// An asynchronous function to be called before starting the server daemon.
    pub(crate) before_start_daemon_hook: Hook,
}

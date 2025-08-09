use crate::*;

/// Main structure for managing server processes.
#[derive(Clone)]
pub struct ServerManager {
    /// Path to the PID file for process tracking.
    pub(crate) pid_file: String,
    /// An asynchronous function to be called before stopping the server.
    pub(crate) stop_hook: Hook,
    /// An asynchronous function to be called before starting the server.
    pub(crate) start_hook: Hook,
    /// Server function to be executed.
    pub(crate) server_hook: Hook,
}

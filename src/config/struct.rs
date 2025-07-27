/// Configuration parameters for server management.
///
/// Holds all necessary configuration values for server operation.
#[derive(Clone)]
pub struct ServerManagerConfig {
    /// Path to the PID file for process tracking.
    pub pid_file: String,
}

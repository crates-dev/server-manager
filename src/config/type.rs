/// Title: ServerConfig structure for configuration parameters
///
/// Parameters:
/// - None
///
/// Returns:
/// - None
///
/// This structure holds the configuration values including the PID file path, stdout log path, and stderr log path.
#[allow(dead_code)]
pub struct ServerConfig {
    pub pid_file: String,
    pub stdout_log: String,
    pub stderr_log: String,
}

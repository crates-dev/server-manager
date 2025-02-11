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
    pub(crate) pid_file: String,
    pub(crate) stdout_log: String,
    pub(crate) stderr_log: String,
}

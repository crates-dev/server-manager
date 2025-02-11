use crate::*;

/// Title: ServerManager structure for managing the server process
///
/// Parameters:
/// - None
///
/// Returns:
/// - None
///
/// This structure encapsulates the server management operations and holds the user-provided configuration and the server function.
pub struct ServerManager<F> {
    pub(crate) config: ServerConfigManager,
    pub(crate) server_fn: F,
}

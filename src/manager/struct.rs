use crate::*;

/// Main structure for managing server processes.
///
/// Encapsulates all server management operations and holds necessary configuration.
#[derive(Clone)]
pub struct ServerManager<F> {
    /// Configuration parameters for server management.
    pub(crate) config: ServerManagerConfig,
    /// Server function to be executed.
    pub(crate) server_fn: F,
}

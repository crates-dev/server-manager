use crate::*;

/// Error type for server management operations.
pub type ServerManagerError = Box<dyn std::error::Error>;

/// Result type for server management operations.
pub type ServerManagerResult = Result<(), ServerManagerError>;

/// Type alias for the hook functions.
pub type Hook = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

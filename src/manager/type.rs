/// Error type for server management operations.
pub type ServerManagerError = Box<dyn std::error::Error>;

/// Result type for server management operations.
pub type ServerManagerResult = Result<(), ServerManagerError>;

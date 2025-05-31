pub type ServerManagerError = Box<dyn std::error::Error>;
pub type ServerManagerResult = Result<(), ServerManagerError>;

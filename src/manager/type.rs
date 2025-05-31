use crate::*;

pub type ServerManagerError = Box<dyn Error>;
pub type ServerManagerResult = Result<(), ServerManagerError>;

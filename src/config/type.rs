use crate::*;

/// Type alias for the hook functions.
pub type Hook = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

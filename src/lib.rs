//! server-manager
//!
//! server-manager is a rust library for managing server processes.
//! It encapsulates service startup, shutdown, and background daemon mode.
//! Users can specify the PID file, log file paths, and other configurations
//! through custom settings, while also passing in their own asynchronous
//! server function for execution. The library supports both synchronous
//! and asynchronous operations. On Unix and Windows platforms,
//! it enables background daemon processes.

mod r#const;
mod r#impl;
mod r#struct;
#[cfg(test)]
mod test;
mod r#type;

pub use {r#struct::*, r#type::*};

use r#const::*;

#[cfg(test)]
use std::time::Duration;
use std::{
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::{Child, Command, ExitStatus, Output, Stdio, exit, id},
    sync::Arc,
};

use tokio::runtime::Runtime;

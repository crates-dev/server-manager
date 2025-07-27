//! server-manager
//!
//! server-manager is a rust library for managing server processes.
//! It encapsulates service startup, shutdown, and background daemon mode.
//! Users can specify the PID file, log file paths, and other configurations
//! through custom settings, while also passing in their own asynchronous
//! server function for execution. The library supports both synchronous
//! and asynchronous operations. On Unix and Windows platforms,
//! it enables background daemon processes.

pub(crate) mod cfg;
pub(crate) mod config;
pub(crate) mod manager;

pub use config::r#struct::*;
pub use manager::{r#struct::*, r#type::*};

pub(crate) use manager::r#const::*;

#[allow(unused_imports)]
pub(crate) use std::{
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Output, Stdio, exit, id},
};

#[allow(unused_imports)]
pub(crate) use tokio::runtime::Runtime;

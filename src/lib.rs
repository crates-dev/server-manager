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
pub(crate) mod manager;

pub use manager::{r#struct::*, r#type::*};

pub(crate) use manager::r#const::*;

pub(crate) use std::{
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::{Child, Command, ExitStatus, Output, Stdio, exit, id},
    sync::Arc,
};

#[cfg(windows)]
pub(crate) use tokio::runtime::Runtime;

#[cfg(not(windows))]
pub(crate) use tokio::runtime::Runtime;

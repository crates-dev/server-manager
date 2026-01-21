//! server-manager
//!
//! server-manager is a rust library for managing server processes.
//! It encapsulates service startup, shutdown, and background daemon mode.
//! Users can specify the PID file, log file paths, and other configurations
//! through custom settings, while also passing in their own asynchronous
//! server function for execution. The library supports both synchronous
//! and asynchronous operations. On Unix and Windows platforms,
//! it enables background daemon processes.

pub(crate) mod r#const;
pub(crate) mod r#impl;
pub(crate) mod r#struct;
pub(crate) mod r#type;

#[cfg(test)]
mod test;

pub use {r#struct::*, r#type::*};

pub(crate) use r#const::*;

pub(crate) use std::{
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::{Child, Command, ExitStatus, Output, Stdio, exit, id},
    sync::Arc,
};

pub(crate) use tokio::runtime::Runtime;

#[cfg(test)]
pub(crate) use std::time::Duration;

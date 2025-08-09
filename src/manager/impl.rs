use crate::*;

impl Default for ServerManager {
    fn default() -> Self {
        let empty_hook: Hook = Arc::new(|| Box::pin(async {}));
        Self {
            pid_file: Default::default(),
            stop_hook: empty_hook.clone(),
            server_hook: empty_hook.clone(),
            start_hook: empty_hook,
        }
    }
}

/// Implementation of server management operations.
///
/// Provides methods for starting, stopping and managing server processes.
impl ServerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pid_file<P: ToString>(&mut self, pid_file: P) -> &mut Self {
        self.pid_file = pid_file.to_string();
        self
    }

    pub fn set_start_hook<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.start_hook = Arc::new(move || Box::pin(f()));
        self
    }

    pub fn set_server_hook<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.server_hook = Arc::new(move || Box::pin(f()));
        self
    }

    pub fn set_stop_hook<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.stop_hook = Arc::new(move || Box::pin(f()));
        self
    }

    pub fn get_pid_file(&self) -> &str {
        &self.pid_file
    }

    pub fn get_start_hook(&self) -> &Hook {
        &self.start_hook
    }

    pub fn get_server_hook(&self) -> &Hook {
        &self.server_hook
    }

    pub fn get_stop_hook(&self) -> &Hook {
        &self.stop_hook
    }

    /// Starts the server in foreground mode.
    ///
    /// Writes the current process ID to the PID file and executes the server function.
    pub async fn start(&self) {
        (self.start_hook)().await;
        if let Err(e) = self.write_pid_file() {
            eprintln!("Failed to write pid file: {}", e);
            return;
        }
        (self.server_hook)().await;
    }

    /// Stops the running server process.
    ///
    /// Reads PID from file and terminates the process.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn stop(&self) -> ServerManagerResult {
        (self.stop_hook)().await;
        let pid: i32 = self.read_pid_file()?;
        self.kill_process(pid)
    }

    /// Starts the server in daemon (background) mode on Unix platforms.
    #[cfg(not(windows))]
    pub async fn start_daemon(&self) -> ServerManagerResult {
        (self.start_hook)().await;
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.server_hook)().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env(RUNNING_AS_DAEMON, RUNNING_AS_DAEMON_VALUE)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        cmd.spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    /// Starts the server in daemon (background) mode on Windows platforms.
    #[cfg(windows)]
    pub async fn start_daemon(&self) -> ServerManagerResult {
        (self.start_hook)().await;
        use std::os::windows::process::CommandExt;
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.server_hook)().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env(RUNNING_AS_DAEMON, RUNNING_AS_DAEMON_VALUE)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .creation_flags(0x00000008);
        cmd.spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(())
    }

    /// Reads process ID from the PID file.
    ///
    /// # Returns
    ///
    /// - `Result<i32, Box<dyn std::error::Error>>` - Process ID if successful.
    fn read_pid_file(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let pid_str: String = fs::read_to_string(&self.pid_file)?;
        let pid: i32 = pid_str.trim().parse::<i32>()?;
        Ok(pid)
    }

    /// Writes current process ID to the PID file.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    fn write_pid_file(&self) -> ServerManagerResult {
        if let Some(parent) = Path::new(&self.pid_file).parent() {
            fs::create_dir_all(parent)?;
        }
        let pid: u32 = id();
        fs::write(&self.pid_file, pid.to_string())?;
        Ok(())
    }

    /// Kills process by PID on Unix platforms.
    ///
    /// # Arguments
    ///
    /// - `i32` - Process ID to terminate.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(not(windows))]
    fn kill_process(&self, pid: i32) -> ServerManagerResult {
        let result: Result<Output, std::io::Error> = Command::new("kill")
            .arg("-TERM")
            .arg(pid.to_string())
            .output();
        match result {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => Err(format!(
                "Failed to kill process with pid: {}, error: {}",
                pid,
                String::from_utf8_lossy(&output.stderr)
            )
            .into()),
            Err(e) => Err(format!("Failed to execute kill command: {}", e).into()),
        }
    }

    /// Kills process by PID on Windows platforms.
    ///
    /// # Arguments
    ///
    /// - `i32` - Process ID to terminate.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    #[cfg(windows)]
    fn kill_process(&self, pid: i32) -> ServerManagerResult {
        use std::ffi::c_void;
        type DWORD = u32;
        type BOOL = i32;
        type HANDLE = *mut c_void;
        type UINT = u32;
        const PROCESS_TERMINATE: DWORD = 0x0001;
        const PROCESS_ALL_ACCESS: DWORD = 0x1F0FFF;
        unsafe extern "system" {
            fn OpenProcess(
                dwDesiredAccess: DWORD,
                bInheritHandle: BOOL,
                dwProcessId: DWORD,
            ) -> HANDLE;
            fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;
            fn CloseHandle(hObject: HANDLE) -> BOOL;
            fn GetLastError() -> DWORD;
        }
        let process_id: DWORD = pid as DWORD;
        let mut process_handle: HANDLE = unsafe { OpenProcess(PROCESS_TERMINATE, 0, process_id) };
        if process_handle.is_null() {
            process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };
        }
        if process_handle.is_null() {
            let error_code = unsafe { GetLastError() };
            return Err(format!(
                "Failed to open process with pid: {}. Error code: {}",
                pid, error_code
            )
            .into());
        }
        let terminate_result: BOOL = unsafe { TerminateProcess(process_handle, 1) };
        if terminate_result == 0 {
            let error_code = unsafe { GetLastError() };
            unsafe {
                CloseHandle(process_handle);
            }
            return Err(format!(
                "Failed to terminate process with pid: {}. Error code: {}",
                pid, error_code
            )
            .into());
        }
        unsafe {
            CloseHandle(process_handle);
        }
        Ok(())
    }

    /// Runs the server with cargo-watch.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - Arguments for cargo-watch.
    /// - `bool` - Whether to wait for process completion.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    async fn run_with_cargo_watch(&self, run_args: &[&str], wait: bool) -> ServerManagerResult {
        (self.start_hook)().await;
        let cargo_watch_installed: Output = Command::new("cargo")
            .arg("install")
            .arg("--list")
            .output()?;
        if !String::from_utf8_lossy(&cargo_watch_installed.stdout).contains("cargo-watch") {
            eprintln!("Cargo-watch not found. Attempting to install...");
            let install_status: ExitStatus = Command::new("cargo")
                .arg("install")
                .arg("cargo-watch")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            if !install_status.success() {
                return Err("Failed to install cargo-watch. Please install it manually: `cargo install cargo-watch`".into());
            }
            eprintln!("Cargo-watch installed successfully.");
        }
        let mut command: Command = Command::new("cargo-watch");
        command
            .args(run_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit());
        let mut child: Child = command
            .spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        if wait {
            child
                .wait()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }
        exit(0);
    }

    /// Starts the server with hot-reloading using cargo-watch.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - Arguments for cargo-watch.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn hot_restart(&self, run_args: &[&str]) -> ServerManagerResult {
        self.run_with_cargo_watch(run_args, false).await
    }

    /// Starts the server with hot-reloading and waits for completion.
    ///
    /// # Arguments
    ///
    /// - `&[&str]` - Arguments for cargo-watch.
    ///
    /// # Returns
    ///
    /// - `ServerManagerResult` - Operation result.
    pub async fn hot_restart_wait(&self, run_args: &[&str]) -> ServerManagerResult {
        self.run_with_cargo_watch(run_args, true).await
    }
}

use crate::*;

impl<F, Fut> ServerManager<F>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    /// Title: Create a new ServerManager instance
    ///
    /// Parameters:
    /// - `config`: The server configuration containing PID file path and log paths.
    /// - `server_fn`: A closure representing the asynchronous server function.
    ///
    /// Returns:
    /// - `ServerManager<F>`: A new instance of ServerManager.
    pub fn new(config: ServerManagerConfig, server_fn: F) -> Self {
        Self { config, server_fn }
    }

    /// Title: Start the server in foreground mode
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `()`: No return value.
    ///
    /// This function writes the current process ID to the PID file specified in the configuration
    /// and then runs the server function asynchronously.
    pub async fn start(&self) {
        if let Err(e) = self.write_pid_file() {
            eprintln!("Failed to write pid file: {}", e);
            return;
        }
        (self.server_fn)().await;
    }

    /// Title: Stop the server
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function reads the process ID from the PID file and attempts to kill the process using a SIGTERM signal.
    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pid: i32 = self.read_pid_file()?;
        self.kill_process(pid)
    }

    /// Start the server in daemon (background) mode on Unix platforms.
    /// Title: Start the server in daemon mode on non-Unix platforms
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function returns an error because daemon mode is not supported on non-Unix platforms.
    #[cfg(not(windows))]
    pub fn start_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.server_fn)().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env(RUNNING_AS_DAEMON, RUNNING_AS_DAEMON_VALUE)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        cmd.spawn()?;
        exit(0);
    }

    #[cfg(windows)]
    /// Start the server in daemon (background) mode on Windows platforms
    /// Title: Start the server in daemon mode on Windows platforms
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function starts a detached process on Windows using Windows API.
    pub fn start_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::windows::process::CommandExt;
        if std::env::var(RUNNING_AS_DAEMON).is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.server_fn)().await;
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
        cmd.spawn()?;
        std::process::exit(0);
    }

    /// Title: Read process ID from the PID file
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<i32, Box<dyn std::error::Error>>`: The process ID if successful.
    ///
    /// This function reads the content of the PID file specified in the configuration and parses it as an integer.    
    fn read_pid_file(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let pid_str: String = fs::read_to_string(&self.config.pid_file)?;
        let pid: i32 = pid_str.trim().parse::<i32>()?;
        Ok(pid)
    }

    /// Title: Write current process ID to the PID file
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function obtains the current process ID and writes it as a string to the PID file specified in the configuration.
    fn write_pid_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = std::path::Path::new(&self.config.pid_file).parent() {
            fs::create_dir_all(parent)?;
        }
        let pid: u32 = id();
        fs::write(&self.config.pid_file, pid.to_string())?;
        Ok(())
    }

    /// Title: Kill process by PID on Unix platforms
    ///
    /// Parameters:
    /// - `pid`: The process ID to kill.
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function sends a SIGTERM signal to the process with the given PID using libc::kill.
    #[cfg(not(windows))]
    fn kill_process(&self, pid: i32) -> Result<(), Box<dyn std::error::Error>> {
        let result: Result<std::process::Output, std::io::Error> = Command::new("kill")
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

    #[cfg(windows)]
    /// Kill process by PID on Windows platforms
    /// Title: Kill process by PID on Windows platforms
    ///
    /// Parameters:
    /// - ``pid``: The process ID to kill.
    ///
    /// Returns:
    /// - ``Result<(), Box<dyn std::error::Error>>``: Operation result.
    ///
    /// This function attempts to kill the process with the given PID using Windows API.
    /// If opening or terminating the process fails, the detailed error code is returned.
    fn kill_process(&self, pid: i32) -> Result<(), Box<dyn std::error::Error>> {
        use std::ffi::c_void;
        type DWORD = u32;
        type BOOL = i32;
        type HANDLE = *mut c_void;
        type UINT = u32;
        const PROCESS_TERMINATE: DWORD = 0x0001;
        const PROCESS_ALL_ACCESS: DWORD = 0x1F0FFF;
        extern "system" {
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
}

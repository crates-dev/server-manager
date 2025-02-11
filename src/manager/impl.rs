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
    #[cfg(unix)]
    pub fn start_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        if std::env::var("RUNNING_AS_DAEMON").is_ok() {
            self.write_pid_file()?;
            let rt: Runtime = Runtime::new()?;
            rt.block_on(async {
                (self.server_fn)().await;
            });
            return Ok(());
        }
        let exe_path: PathBuf = std::env::current_exe()?;
        let mut cmd: Command = Command::new(exe_path);
        cmd.env("RUNNING_AS_DAEMON", "1")
            .stdout(Stdio::from(fs::File::create(&self.config.stdout_log)?))
            .stderr(Stdio::from(fs::File::create(&self.config.stderr_log)?))
            .stdin(Stdio::null());
        cmd.spawn()?;
        exit(0);
    }

    /// 在非 Unix 平台下返回错误。
    #[cfg(not(unix))]
    pub fn start_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        Err("Daemon mode is not supported on non-Unix platforms".into())
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
    #[cfg(unix)]
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

    /// Title: Kill process by PID on non-Unix platforms
    ///
    /// Parameters:
    /// - `pid`: The process ID to kill.
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function returns an error because killing a process is not supported on non-Unix platforms.

    #[cfg(not(unix))]
    fn kill_process(&self, _pid: i32) -> Result<(), Box<dyn std::error::Error>> {
        Err("kill_process is not supported on non-Unix platforms".into())
    }
}

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
    pub fn new(config: ServerManagerConfigConfig, server_fn: F) -> Self {
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
    pub fn stop(&self) -> Result<(), Box<dyn Error>> {
        let pid: i32 = self.read_pid_file()?;
        self.kill_process(pid)
    }

    /// Title: Start the server in daemon (background) mode on Unix platforms
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function uses the daemonize crate to run the server process in the background. It configures
    /// the PID file, stdout log, and stderr log paths from the configuration.
    #[cfg(unix)]
    pub fn start_daemon(&self) -> Result<(), Box<dyn Error>> {
        let stdout_file = fs::File::create(&self.config.stdout_log)?;
        let stderr_file = fs::File::create(&self.config.stderr_log)?;
        let daemonize_obj = Daemonize::new()
            .pid_file(&self.config.pid_file)
            .chown_pid_file(true)
            .working_directory(".")
            .umask(0o027)
            .stdout(stdout_file)
            .stderr(stderr_file);
        daemonize_obj.start()?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            (self.server_fn)().await;
        });
        Ok(())
    }

    /// Title: Start the server in daemon mode on non-Unix platforms
    ///
    /// Parameters:
    /// - None
    ///
    /// Returns:
    /// - `Result<(), Box<dyn std::error::Error>>`: Operation result.
    ///
    /// This function returns an error because daemon mode is not supported on non-Unix platforms.
    #[cfg(not(unix))]
    pub fn start_daemon(&self) -> Result<(), Box<dyn Error>> {
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
    fn read_pid_file(&self) -> Result<i32, Box<dyn Error>> {
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
    fn write_pid_file(&self) -> Result<(), Box<dyn Error>> {
        let pid: u32 = process::id();
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
    fn kill_process(&self, pid: i32) -> Result<(), Box<dyn Error>> {
        let result = unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
        if result == 0 {
            Ok(())
        } else {
            Err(format!("Failed to kill process with pid: {}", pid).into())
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
    fn kill_process(&self, _pid: i32) -> Result<(), Box<dyn Error>> {
        Err("kill_process is not supported on non-Unix platforms".into())
    }
}

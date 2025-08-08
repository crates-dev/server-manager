use crate::*;

impl ServerManagerConfig {
    pub fn new(pid_file: String) -> Self {
        Self {
            pid_file,
            ..Default::default()
        }
    }

    pub fn set_pid_file(&mut self, pid_file: String) -> &mut Self {
        self.pid_file = pid_file;
        self
    }

    pub fn set_before_stop_hook<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.before_stop_hook = Arc::new(move || Box::pin(f()));
        self
    }

    pub fn set_before_start_daemon_hook<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.before_start_daemon_hook = Arc::new(move || Box::pin(f()));
        self
    }

    pub fn get_pid_file(&self) -> &str {
        &self.pid_file
    }

    pub fn get_before_stop_hook(&self) -> &Hook {
        &self.before_stop_hook
    }

    pub fn get_before_start_daemon_hook(&self) -> &Hook {
        &self.before_start_daemon_hook
    }
}

impl Default for ServerManagerConfig {
    fn default() -> Self {
        let empty_hook: Hook = Arc::new(|| Box::pin(async {}));
        Self {
            pid_file: Default::default(),
            before_stop_hook: empty_hook.clone(),
            before_start_daemon_hook: empty_hook,
        }
    }
}

#[tokio::test]
async fn test_start_executes_server_fn() {
    use crate::*;
    use std::fs;
    use std::time::Duration;
    let pid_file: String = "test_pid.pid".to_string();
    let _ = fs::remove_file(&pid_file);
    let config: ServerManagerConfigConfig = ServerManagerConfigConfig {
        pid_file: pid_file.clone(),
        stdout_log: "test_stdout.log".to_string(),
        stderr_log: "test_stderr.log".to_string(),
    };
    let dummy_server = || async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    };
    let manager = ServerManager::new(config, dummy_server);
    let res: Result<(), Box<dyn Error>> = manager.start_daemon();
    println!("start_daemon {:?}", res);
    let res: Result<(), Box<dyn Error>> = manager.stop();
    println!("stop {:?}", res);
    manager.start().await;
    let _ = fs::remove_file(&pid_file);
}

#[tokio::test]
async fn test_start_executes_server_fn() {
    use crate::*;
    use std::fs;
    use std::time::Duration;

    let pid_file: String = "./process/test_pid.pid".to_string();
    let _ = fs::remove_file(&pid_file);
    let config: ServerManagerConfig = ServerManagerConfig {
        pid_file: pid_file.clone(),
    };
    let dummy_server = || async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    };
    let manager = ServerManager::new(config, dummy_server);
    let res: ServerManagerResult = manager.start_daemon();
    println!("start_daemon {:?}", res);
    let res: ServerManagerResult = manager.stop();
    println!("stop {:?}", res);
    manager.start().await;
    let _ = fs::remove_file(&pid_file);
}

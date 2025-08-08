#[tokio::test]
async fn test_start_executes_server_fn() {
    use crate::*;
    use std::fs;
    use std::time::Duration;

    let pid_file: String = "./process/test_pid.pid".to_string();
    let _ = fs::remove_file(&pid_file);
    let mut config = ServerManagerConfig::new(pid_file.clone());
    config
        .set_start_hook(|| async {
            println!("Before start daemon hook executed");
        })
        .set_stop_hook(|| async {
            println!("Before stop hook executed");
        });
    let server = || async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    };
    let manager = ServerManager::new(config, server);
    let res: ServerManagerResult = manager.start_daemon().await;
    println!("start_daemon {:?}", res);
    let res: ServerManagerResult = manager.stop().await;
    println!("stop {:?}", res);
    manager.start().await;
    let _ = fs::remove_file(&pid_file);
}

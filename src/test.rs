use crate::*;

#[tokio::test]
async fn test_start_executes_server_fn() {
    let pid_file: String = "./process/test_pid.pid".to_string();
    let _ = fs::remove_file(&pid_file);
    let server = || async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    };
    let mut manager: ServerManager = ServerManager::new();
    manager
        .set_pid_file(&pid_file)
        .set_start_hook(|| async {
            println!("Before start daemon hook executed");
        })
        .set_server_hook(server)
        .set_stop_hook(|| async {
            println!("Before stop hook executed");
        });
    let res: ServerManagerResult = manager.start_daemon().await;
    println!("start_daemon {res:?}");
    let res: ServerManagerResult = manager.stop().await;
    println!("stop {res:?}");
    manager.start().await;
    let _ = fs::remove_file(&pid_file);
}

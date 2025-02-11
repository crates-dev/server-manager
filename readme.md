<center>

## server-manager

[![](https://img.shields.io/crates/v/server-manager.svg)](https://crates.io/crates/server-manager)
[![](https://docs.rs/server-manager/badge.svg)](https://docs.rs/server-manager)
[![](https://github.com/ltpp-universe/server-manager/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/server-manager/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/server-manager.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/server-manager/)

[Api Docs](https://docs.rs/server-manager/latest/server_manager/)

> `server-manager` is a Rust library for managing server processes. It encapsulates service startup, shutdown, and background daemon mode. Users can specify the PID file, log file paths, and other configurations through custom settings, while also passing in their own asynchronous server function for execution. The library supports both synchronous and asynchronous operations. On Unix platforms, it enables background daemon processes, while on non-Unix platforms, it returns an appropriate error message.

## Installation

To use this crate, you can run cmd:

```shell
cargo add server-manager
```

## Use

```rust
use server_manager::*;
use std::fs;
use std::time::Duration;
let pid_file: String = "test_pid.pid".to_string();
let _ = fs::remove_file(&pid_file);
let config: ServerManagerConfig = ServerManagerConfig {
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).

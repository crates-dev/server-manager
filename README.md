<center>

## server-manager

[![](https://img.shields.io/crates/v/server-manager.svg)](https://crates.io/crates/server-manager)
[![](https://img.shields.io/crates/d/server-manager.svg)](https://img.shields.io/crates/d/server-manager.svg)
[![](https://docs.rs/server-manager/badge.svg)](https://docs.rs/server-manager)
[![](https://github.com/crates-dev/server-manager/workflows/Rust/badge.svg)](https://github.com/crates-dev/server-manager/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/server-manager.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/server-manager/)

[Api Docs](https://docs.rs/server-manager/latest/)

> server-manager is a rust library for managing server processes. It encapsulates service startup, shutdown, and background daemon mode. Users can specify the PID file, log file paths, and other configurations through custom settings, while also passing in their own asynchronous server function for execution. The library supports both synchronous and asynchronous operations. On Unix and Windows platforms, it enables background daemon processes.

## Installation

To use this crate, you can run cmd:

```shell
cargo add server-manager
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).

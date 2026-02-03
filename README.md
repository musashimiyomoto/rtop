# Rust Top (Multi-Disk)

A lightweight, terminal-based system monitoring tool written in Rust. It displays real-time CPU usage, Memory usage, and Disk space information for all available drives.

## Features

- **CPU Load**: Shows current CPU utilization.
- **Memory Usage**: Displays used and total RAM.
- **Disk Usage**: Monitors free/used space for all logical drives (C:, D:, etc.).
- **Process Count**: Shows total number of running processes.
- **Cross-Platform**: Built generic where possible, but currently uses Windows-specific commands (`wmic`).

## Prerequisites

- **Rust**: You need to have Rust installed. If you don't have it, get it from [rustup.rs](https://rustup.rs/).

## Building and Running

### Windows

1.  **Clone the repository**:
    ```bash
    git clone <repository-url>
    cd rust_top
    ```

2.  **Build the project**:
    ```bash
    cargo build --release
    ```

3.  **Run**:
    ```bash
    cargo run --release
    ```
    Or run the executable directly from `target/release/rust_top.exe`.

### Linux & macOS

*Note: The current implementation heavily relies on Windows Management Instrumentation (WMIC). Linux and macOS support requires adapting the system calls.*

1.  **Build**:
    The code compiles on all platforms, as it uses standard Rust code.
    ```bash
    cargo build --release
    ```

2.  **Run**:
    ```bash
    cargo run --release
    ```
    *Warning: You may see errors or empty stats if `wmic` is not available on your system (which is tailored for Windows).*

## Development & Testing

We use standard `cargo` commands for testing. The project includes unit tests for the parsing logic to ensure stability.

To run tests:
```bash
cargo test
```

## License

MIT

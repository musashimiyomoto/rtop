# Rust Top (Multi-Disk)

A lightweight, terminal-based system monitoring tool written in Rust. It displays real-time CPU usage, Memory usage, and Disk space information for all available drives.

## Features

- **CPU Load**: Shows current CPU utilization.
- **Memory Usage**: Displays used and total RAM.
- **Disk Usage**: Monitors free/used space for all logical drives (C:, D:, etc.).
- **Process Count**: Shows total number of running processes.
- **Cross-Platform**: Full support for Windows, Linux, and macOS using `sysinfo`.

## Prerequisites

- **Rust**: You need to have Rust installed. If you don't have it, get it from [rustup.rs](https://rustup.rs/).

## Building and Running

### All Platforms

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

## Development & Testing

We use standard `cargo` commands for testing. The project includes unit tests for the parsing logic to ensure stability.

To run tests:
```bash
cargo test
```

## License

MIT

# adl-rs

[![CI (Windows)](https://github.com/shm11C3/adl-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shm11C3/adl-rs/actions/workflows/ci.yml)

This repository provides Rust bindings for the AMD Display Library (ADL).  
The project is currently under active development and aims to provide complete coverage of the ADL SDK.

## Develop

### Directory Structure

```text
adl-rs
├── adl
│   ├── src
│   ├── Cargo.toml
│   └── Cargo.lock
├── adl-example
│   ├── src
│   ├── Cargo.toml
│   └── Cargo.lock
├── adl-sys
│   ├── src
│   ├── Cargo.toml
│   └── Cargo.lock
├── Cargo.toml
└── Cargo.lock
```

- `adl`: The high-level library that provides safe and ergonomic Rust bindings for ADL.
- `adl-sys`: The low-level library that provides raw FFI bindings to the ADL SDK.
- `adl-example`: A sample application demonstrating how to use the adl crate.

### Build

```bash
cargo build --workspace
```

### Run

```bash
cargo run --manifest-path ./adl-example/Cargo.toml
```

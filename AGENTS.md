# Agent Instructions for `moon_toolchain_nix`

This repository contains a **Moonrepo plugin** for the **Nix toolchain**, written in **Rust**.
It supports Nix Flakes, Flox, and devenv environments.

## Project Overview

- **Language:** Rust (edition 2021)
- **Target:** `wasm32-wasip1` (compiled to WASM for Moonrepo plugin system)
- **Key Dependencies:** `moon_pdk`, `extism-pdk`, `schematic`, `serde`
- **Purpose:** A Moonrepo plugin to detect Nix environments (flake.nix, shell.nix, devenv.nix, etc.) and configure the toolchain accordingly.

## Directory Structure

- `src/lib.rs`: Entry point.
- `src/config.rs`: Configuration definition (`NixToolchainConfig`).
- `src/nix.rs`: Main implementation logic (detects environments, sets up commands).
- `examples/`: Example configurations for different Nix environments (Flakes, Flox, Devenv).
- `moon.yml`: Moonrepo project configuration.
- `rust-toolchain.toml`: Specifies Rust version (1.85.0).
- `devenv.nix`, `devenv.yaml`: Configuration for the development environment.

## Build Instructions

To build the plugin (WASM):

```bash
cargo build --target wasm32-wasip1
```

To build for release:

```bash
cargo build --release --target wasm32-wasip1
```

The output WASM file will be in `target/wasm32-wasip1/debug/` or `target/wasm32-wasip1/release/`.

## Testing

Currently, there are no explicit tests in the `src` directory or a `tests` directory, although `dev-dependencies` include testing utilities (`moon_pdk_test_utils`, `starbase_sandbox`).

When adding features, consider adding tests if possible, potentially using these utilities.

## Development Environment & Dependencies (IMPORTANT)

This project strictly uses **Nix** (via `devenv`) to manage all dependencies and the development environment.

- **Do NOT** use system package managers (like `apt`, `brew`, `pacman`) to install dependencies.
- **Do NOT** modify your local environment manually.
- All dependencies should be defined in `devenv.nix` or `devenv.yaml`.
- The Rust toolchain is managed by `devenv` (which uses `rust-toolchain.toml`).
- To activate the environment locally, use `devenv shell` or allow `direnv` if configured.

## Moon Tasks

Defined in `moon.yml`:
- `build`: Builds the debug WASM (`cargo build --target wasm32-wasip1`).
- `build-release`: Builds the release WASM (`cargo build --release --target wasm32-wasip1`).
- `check`: Runs `cargo check --target wasm32-wasip1`.
- `clippy`: Runs `cargo clippy --target wasm32-wasip1 -- -D warnings`.
- `format`: Runs `cargo fmt`.
- `fmt-check`: Runs `cargo fmt -- --check`.
- `clean`: Runs `cargo clean`.

## Guidelines

1. **Use Nix for Dependencies:** Always check `devenv.nix` if you need new tools or libraries. Do not assume system availability.
2. **Verify Changes:** Always compile the project after changes (`cargo build --target wasm32-wasip1`) to ensure no regressions.
3. **Code Style:** Follow Rust idioms and run `cargo fmt` if modifying code.
4. **WASM Target:** Remember this code compiles to WASM/WASI, so some standard library features might be limited or behave differently.

# rustos

`rustos` is a minimal, educational, and maintainable open-source project for learning the fundamentals of the Rust ecosystem and modern operating system development by building a small OS from scratch.

The project favors clarity over cleverness, small steps over large abstractions, and explicit design decisions over hidden complexity.

## Goals

- Build a minimal bootable kernel in Rust
- Establish a clean Unix-like OS foundation
- Keep the codebase simple, readable, and easy to contribute to
- Teach OS fundamentals through real code and concise documentation
- Follow modern open-source engineering practices from the start

## Non-Goals

For the MVP, `rustos` does not aim to provide:

- a production-ready operating system
- broad hardware support
- a full Unix-compatible userland
- networking, filesystems, or multitasking from day one
- unnecessary abstraction or framework-heavy design

## MVP Scope

The initial MVP focuses on:

- `x86_64-unknown-uefi` as the target platform
- running on Apple Silicon hosts through QEMU
- a minimal Rust `no_std` kernel foundation
- a reproducible local development workflow
- concise architecture and roadmap documentation
- contributor-friendly repository structure and project hygiene

## Project Principles

- Minimal first
- Code-first, concise docs
- Explicit unsafe boundaries
- Small modules with clear responsibilities
- Reproducible developer workflows
- Open-source friendly by default

## Repository Layout

This repository is organized as a small monorepo:

- `kernel/` — kernel crate and OS code
- `xtask/` — developer workflow commands
- `docs/` — roadmap, architecture notes, and design decisions
- `.github/` — CI and contribution templates

## Status

This project is in the early foundation stage.

Current milestone:
- Milestone 2 — developer workflow and CI

## Roadmap

See:
- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/unix-like.md`
- `docs/decisions/0001-target-platform.md`

## Local Development

Milestone 2 focuses on making the project easier to run, validate, and contribute to while keeping the workflow minimal and explicit.

### Requirements

Install:

- stable Rust
- the `x86_64-unknown-uefi` Rust target
- QEMU with `qemu-system-x86_64`

The Rust toolchain and target are pinned in `rust-toolchain.toml`.

#### Suggested macOS Setup

If you are developing on macOS, install the emulator tools you need first.

For example, with Homebrew:

- `brew install qemu`

You do not need `uefi-run`.

### Recommended Workflow

Use `xtask` as the main entry point for common project tasks.

The current workflow is:

1. `cargo run -p xtask -- check`
2. `cargo run -p xtask -- fmt`
3. `cargo run -p xtask -- lint`
4. `cargo run -p xtask -- run-test`
5. `cargo run -p xtask -- run` when you want an interactive QEMU session

This keeps local development and CI aligned around the same commands.

### Common Commands

Check the workspace:

- `cargo run -p xtask -- check`

Check formatting:

- `cargo run -p xtask -- fmt`

Run lints:

- `cargo run -p xtask -- lint`

Run a bounded boot test:

- `cargo run -p xtask -- run-test`

Build and run the UEFI application interactively:

- `cargo run -p xtask -- run`

### What `xtask` Does

The current `xtask` commands are:

- `check` — runs `cargo check --workspace --all-targets`
- `fmt` — checks formatting with `rustfmt`
- `lint` — runs `clippy` with warnings denied
- `run-test` — launches QEMU in bounded test mode and exits automatically after success or timeout
- `run` — launches QEMU interactively for manual inspection

The boot commands:

1. build the `kernel` crate for `x86_64-unknown-uefi`
2. create a small EFI boot directory
3. copy the generated binary to `EFI/BOOT/BOOTX64.EFI`
4. write a `startup.nsh` script that launches `EFI\\BOOT\\BOOTX64.EFI`
5. launch QEMU directly with explicit UEFI firmware files
6. forward extra arguments to QEMU

Example with extra QEMU arguments:

- `cargo run -p xtask -- run -m 512M`

### Bounded Run Mode

For local interactive use, `cargo run -p xtask -- run` keeps QEMU attached so you can inspect the boot flow manually.

For editor agents, CI, and sandboxed environments, use:

- `cargo run -p xtask -- run-test`

This mode:

- captures QEMU output
- waits for the expected boot marker
- exits automatically after success
- fails clearly if the expected output is not observed before timeout

You can change the timeout with:

- `RUSTOS_QEMU_TIMEOUT_SECS=15 cargo run -p xtask -- run-test`

This keeps the default developer experience simple while making automated validation faster and less likely to hang.

### Firmware Notes

The direct QEMU workflow is intentionally explicit.

By default, the project looks for common firmware files such as:

- `edk2-x86_64-code.fd`
- `OVMF_CODE.fd`
- `OVMF_VARS.fd`

If your local firmware files are in a different location, set:

- `RUSTOS_UEFI_CODE`
- `RUSTOS_UEFI_VARS`

### Current Boot Behavior

The current Milestone 1 boot path is intentionally small.

It:

- enters through a UEFI entry point
- initializes UEFI helper support
- prints deterministic boot messages
- exits successfully

This is a bootable foundation, not yet a full kernel runtime.

### Notes

- Local execution depends on QEMU and UEFI firmware files being installed correctly.
- CI should use the same `xtask` commands as local development whenever practical.
- The direct QEMU workflow is preferred because it is simpler to understand and easier to debug than a wrapper-based runner.
- The boot directory includes a `startup.nsh` script so the UEFI shell can launch `BOOTX64.EFI` automatically.
- `run-test` is the preferred command for automated environments because it does not hang indefinitely.
- More detailed boot and debugging guidance can be added once the run path is stable across environments.

## Contributing

Contributions, questions, and suggestions are welcome.

Please start with:
- `CONTRIBUTING.md`

When contributing, prefer:
- small focused changes
- clear commit messages
- simple and readable code
- concise documentation updates when behavior or design changes

Before opening a pull request, run:

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`

If your change affects boot behavior or the QEMU workflow, also run:

- `cargo run -p xtask -- run-test`

## License

MIT

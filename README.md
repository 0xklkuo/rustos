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
- Milestone 1 — minimal bootable UEFI kernel

## Roadmap

See:
- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/decisions/0001-target-platform.md`

## Local Development

Milestone 1 introduces the first bootable UEFI application and the first real `xtask` workflow.

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

### Common Commands

Check the workspace:

- `cargo run -p xtask -- check`

Check formatting:

- `cargo run -p xtask -- fmt`

Run lints:

- `cargo run -p xtask -- lint`

Build and run the UEFI application:

- `cargo run -p xtask -- run`

### What `xtask run` Does

The `run` command:

1. builds the `kernel` crate for `x86_64-unknown-uefi`
2. creates a small EFI boot directory
3. copies the generated binary to `EFI/BOOT/BOOTX64.EFI`
4. writes a `startup.nsh` script that launches `EFI\BOOT\BOOTX64.EFI`
5. creates a FAT disk image with the host image tool
6. launches QEMU directly with explicit UEFI firmware files
7. forwards extra arguments to QEMU

Example with extra QEMU arguments:

- `cargo run -p xtask -- run -m 512M`

### Bounded Run Mode

For local interactive use, `cargo run -p xtask -- run` should keep QEMU attached so you can inspect the boot flow manually.

For editor agents, CI, and sandboxed environments, the run workflow should also support a bounded mode that exits automatically. A practical design is:

- a timeout-based mode, such as `cargo run -p xtask -- run --timeout-secs 5`
- an output-based mode that exits after a known boot message is observed
- a non-interactive test mode that fails clearly if the expected boot output is not produced

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

- Local execution depends on QEMU, UEFI firmware files, and the host image tool being installed correctly.
- CI validates that the UEFI target builds, but it does not guarantee your local emulator setup.
- The direct QEMU workflow is preferred because it is simpler to understand and easier to debug than a wrapper-based runner.
- The boot image now includes a `startup.nsh` script so the UEFI shell can launch `BOOTX64.EFI` automatically.
- A bounded run mode is recommended for automated environments so test runs do not hang indefinitely.
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

## License

MIT

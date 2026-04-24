# rustos

[![CI](https://github.com/0xklkuo/rustos/actions/workflows/ci.yml/badge.svg)](https://github.com/0xklkuo/rustos/actions/workflows/ci.yml) [![Release](https://img.shields.io/github/v/release/0xklkuo/rustos?label=release&sort=semver)](https://github.com/0xklkuo/rustos/releases) [![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE) [![Rust nightly](https://img.shields.io/badge/rust-nightly-blue.svg)](./rust-toolchain.toml) [![Target: x86_64 UEFI](https://img.shields.io/badge/target-x86_64--unknown--uefi-blueviolet.svg)](./docs/decisions/0001-target-platform.md)

`rustos` is a minimal, educational, and maintainable open-source project for learning Rust systems programming and modern operating system fundamentals by building a small OS from scratch.

The project favors clarity over cleverness, small steps over large abstractions, and explicit design decisions over hidden complexity.

## Goals

- Build a minimal bootable kernel in Rust
- Establish a clean Unix-like OS foundation
- Keep the codebase simple, readable, and easy to contribute to
- Teach OS fundamentals through real code and concise documentation
- Follow modern open-source engineering practices from the start

## Non-Goals

For the current MVP, `rustos` does not aim to provide:

- a production-ready operating system
- broad hardware support
- a full Unix-compatible userland
- multitasking, filesystems, or networking
- unnecessary abstraction or framework-heavy design

## Current Status

`rustos` is in an early, foundation-first stage.

Current focus:
- U6 — Unix-like kernel boundary

What works today:
- boots as a small `x86_64-unknown-uefi` kernel in QEMU
- prints deterministic boot and runtime logs
- includes bounded QEMU smoke tests and host-side unit tests
- keeps pure logic in `nucleus/` and firmware-facing runtime code in `kernel/`

## Proof of Life

A successful bounded QEMU boot currently looks like this:

```text
rustos: boot start
rustos: hello from UEFI
rustos: boot mode normal
rustos: runtime init start
rustos: console init complete
rustos: arch init start
x86_64
arch runtime ready
rustos: arch init complete
rustos: exception init
exception groundwork ready
rustos: exception groundwork modeled
rustos: interrupt init
timer interrupt groundwork ready
rustos: interrupt groundwork modeled
rustos: timer init
rustos: timer groundwork modeled
rustos: memory init
rustos: memory map init
rustos: discovered conventional memory
rustos: first conventional range discovered
rustos: frame allocator init
rustos: frame allocator seed ready
rustos: memory foundation ready
rustos: paging init
rustos: paging direction defined
rustos: paging arch probe ready
rustos: heap init deferred
rustos: syscall init
rustos: syscall direction defined
rustos: syscall boundary ready
rustos: panic
rustos: idle ready
rustos: runtime init complete
```

## Quick Start

### Requirements

Install:

- nightly Rust
- the `x86_64-unknown-uefi` Rust target
- QEMU with `qemu-system-x86_64`

The Rust toolchain and target are pinned in `rust-toolchain.toml`.

On macOS, for example:

- `brew install qemu`

### Common Commands

Use `xtask` as the main entry point for local development.

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`
- `cargo run -p xtask -- test-unit`
- `cargo run -p xtask -- test-qemu`
- `cargo run -p xtask -- test-exception`
- `cargo run -p xtask -- run`

If you want the standard combined local validation flow, run:

- `cargo run -p xtask -- test`

## Repository Layout

This repository is organized as a small monorepo:

- `kernel/` — kernel crate and low-level OS code
- `nucleus/` — host-testable pure logic shared with the kernel
- `xtask/` — developer workflow commands
- `docs/` — architecture notes, roadmap, and subsystem direction docs
- `.github/` — CI and contribution templates

## Documentation Map

Start here:

- `docs/README.md` — documentation guide
- `docs/architecture.md` — stable architecture and project principles
- `docs/roadmap.md` — milestone status and implementation sequence
- `docs/testing.md` — testing strategy and validation workflow
- `docs/unix-like.md` — Unix-like direction for the project

Subsystem notes:

- `docs/paging.md`
- `docs/syscalls.md`
- `docs/tasks.md`
- `docs/descriptors.md`
- `docs/blog-os-adoption.md`
- `docs/decisions/0001-target-platform.md`

## Releases

Release planning and release-facing changes are tracked in:

- `CHANGELOG.md`
- GitHub Releases

The current release preparation target is:

- `v0.1.0-alpha.1`

## Contributing

Contributions, questions, and suggestions are welcome.

Please start with:
- `CONTRIBUTING.md`

Before opening a pull request, run:

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`
- `cargo run -p xtask -- test-unit`

If your change affects boot behavior or the QEMU workflow, also run:

- `cargo run -p xtask -- test-qemu`

If your change affects the controlled exception path, also run:

- `cargo run -p xtask -- test-exception`

## License

Licensed under the MIT License. See `LICENSE`.

# rustos

[![CI](https://github.com/0xklkuo/rustos/actions/workflows/ci.yml/badge.svg)](https://github.com/0xklkuo/rustos/actions/workflows/ci.yml) [![Release](https://img.shields.io/github/v/release/0xklkuo/rustos?label=release&sort=semver)](https://github.com/0xklkuo/rustos/releases) [![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE) [![Rust nightly](https://img.shields.io/badge/rust-nightly-blue.svg)](./rust-toolchain.toml) [![Target: x86_64 UEFI](https://img.shields.io/badge/target-x86_64--unknown--uefi-blueviolet.svg)](./docs/decisions/0001-target-platform.md)

`rustos` is a minimal, educational, UEFI-first operating system project written in Rust.

The project is intentionally small, but it is no longer just a scaffold. It boots in QEMU, exercises a real breakpoint-handler path, discovers the UEFI memory map, and keeps most pure logic in host-testable modules so the foundation stays understandable and maintainable.

## What exists today

- a bootable `x86_64-unknown-uefi` kernel for QEMU
- deterministic early-boot and runtime log output
- a real bounded breakpoint exception smoke path
- real UEFI memory-map discovery and a derived frame-allocator seed
- host-testable pure logic in `nucleus/` for:
  - `arch`
  - `console`
  - `interrupt`
  - `memory`
  - `paging`
  - `syscall`
  - `task`
  - `descriptor`
  - `vfs`
- explicit kernel boundaries in `kernel/` for boot, architecture, paging, syscalls, and the new VFS starter
- a Rust-native `xtask` workflow for checks, formatting, linting, unit tests, QEMU smoke tests, and local runs

## What is intentionally deferred

`rustos` is still a foundation-first project. It does **not** currently provide:

- user-mode execution
- a scheduler or multitasking
- real frame allocation
- a heap allocator
- page-table management or mapping APIs
- descriptor tables
- a real virtual filesystem implementation
- filesystems or networking
- broad hardware support or multi-architecture support
- POSIX compatibility claims

## Quick start

### Requirements

Install:

- nightly Rust
- the `x86_64-unknown-uefi` Rust target
- QEMU with `qemu-system-x86_64`

The toolchain is pinned in `rust-toolchain.toml`.

On macOS:

- `brew install qemu`

### Common commands

Use `xtask` as the main local entry point:

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`
- `cargo run -p xtask -- test-unit`
- `cargo run -p xtask -- test-qemu`
- `cargo run -p xtask -- test-exception`
- `cargo run -p xtask -- test`
- `cargo run -p xtask -- run`

## Repository layout

- `kernel/` — firmware-facing kernel code and runtime boundaries
- `nucleus/` — host-testable pure logic and small subsystem models
- `xtask/` — developer workflow commands
- `docs/` — project contract, architecture, roadmap, and decisions
- `.github/` — CI and contribution templates

## Core docs

The documentation is intentionally centered on four core docs:

- `README.md` — project overview and onboarding
- `docs/spec.md` — current technical contract and subsystem behavior
- `docs/architecture.md` — codebase structure, boundaries, and design rules
- `docs/roadmap.md` — status, sequencing, and near-term priorities

Durable platform rationale lives in:

- `docs/decisions/0001-target-platform.md`

## Proof of life

The normal boot path currently emits this sequence:

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
rustos: vfs init
rustos: vfs namespace ready
rustos: vfs console path ready
rustos: panic
rustos: idle ready
rustos: runtime init complete
```

## Contributing

Start with:

- `CONTRIBUTING.md`
- `docs/spec.md`
- `docs/architecture.md`
- `docs/roadmap.md`

## License

Licensed under the MIT License. See `LICENSE`.

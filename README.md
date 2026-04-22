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
- `nucleus/` — host-testable pure logic shared with the kernel
- `xtask/` — developer workflow commands
- `docs/` — roadmap, architecture notes, and design decisions
- `.github/` — CI and contribution templates

## Status

This project is in the early foundation stage.

Current milestone:
- Milestone U2 — exception and interrupt groundwork

## Roadmap

See:
- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/unix-like.md`
- `docs/testing.md`
- `docs/blog-os-adoption.md`
- `docs/decisions/0001-target-platform.md`

## Local Development

The current workflow focuses on keeping validation explicit, minimal, and aligned between local development and CI.

### Requirements

Install:

- nightly Rust
- the `x86_64-unknown-uefi` Rust target
- QEMU with `qemu-system-x86_64`

The Rust toolchain and target are pinned in `rust-toolchain.toml`.

Nightly is currently required for the first real x86_64 breakpoint handler path because the `x86-interrupt` ABI is still unstable. Keep the nightly-only surface as small as possible and prefer stable-compatible code everywhere else.

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
4. `cargo run -p xtask -- test-unit`
5. `cargo run -p xtask -- test-qemu`
6. `cargo run -p xtask -- test-exception`
7. `cargo run -p xtask -- run` when you want an interactive QEMU session

`test-exception` is intended to boot the kernel in an explicit exception-test mode. The next controlled-exception milestone is to validate a real breakpoint-first exception path with a handler-originated success marker.

Nightly is used carefully here because the real breakpoint handler path depends on unstable Rust support for the `x86-interrupt` ABI. This should remain tightly scoped to the low-level exception boundary so the rest of the project stays as stable and maintainable as possible.

`test-unit` is intended to cover host-testable pure logic, which should increasingly live in `nucleus/` instead of the UEFI-facing `kernel/` crate.

This keeps local development and CI aligned around the same commands.

### Common Commands

Check the workspace:

- `cargo run -p xtask -- check`

Check formatting:

- `cargo run -p xtask -- fmt`

Run lints:

- `cargo run -p xtask -- lint`

Run host-side unit tests:

- `cargo run -p xtask -- test-unit`

Run a bounded QEMU boot test:

- `cargo run -p xtask -- test-qemu`

Run a bounded exception smoke test:

- `cargo run -p xtask -- test-exception`

Run the full local test flow:

- `cargo run -p xtask -- test`

Build and run the UEFI application interactively:

- `cargo run -p xtask -- run`

### What `xtask` Does

The current `xtask` commands are:

- `check` — runs `cargo check --workspace --all-targets`
- `fmt` — checks formatting with `rustfmt`
- `lint` — runs `clippy` with warnings denied
- `test-unit` — runs host-side unit tests for workspace crates, especially pure logic in `nucleus/`
- `test-qemu` — launches QEMU in bounded test mode and exits automatically after success or timeout
- `test-exception` — launches a bounded exception smoke test in QEMU using an explicit exception-test boot mode and is intended to validate the real breakpoint handler milestone once that path is complete
- `test` — runs the unit-test flow first, then the bounded QEMU test
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

### Split Test Workflow

For local interactive use, `cargo run -p xtask -- run` keeps QEMU attached so you can inspect the boot flow manually.

For automated validation, the project now uses a split test workflow:

- `cargo run -p xtask -- test-unit`
- `cargo run -p xtask -- test-qemu`
- `cargo run -p xtask -- test-exception`

Use:

- `cargo run -p xtask -- test`

when you want the standard combined local test flow.

The bounded QEMU test mode:

- captures QEMU output
- waits for the expected boot marker
- exits automatically after success
- fails clearly if the expected output is not observed before timeout

You can change the timeout with:

- `RUSTOS_QEMU_TIMEOUT_SECS=15 cargo run -p xtask -- test-qemu`

This keeps fast tests separate from emulator tests while preserving a simple combined command for contributors.

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
- `test-qemu` is the preferred command for automated environments because it does not hang indefinitely.
- `test-exception` should be used when validating the controlled exception path separately from the normal boot smoke test.
- the controlled exception path is moving from scaffolded reporting to a real breakpoint-handler milestone.
- the target end state for this milestone is a handler-originated success marker instead of a post-trigger scaffold marker.
- nightly is currently required for that low-level handler boundary, so compatibility should be reviewed carefully whenever kernel exception dependencies change.
- `test-unit` is intended for fast host-side feedback before running emulator-based validation.
- Host-testable pure logic should prefer `nucleus/`, while firmware-facing runtime code should remain in `kernel/`.
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
- `cargo run -p xtask -- test-unit`

If your change affects boot behavior or the QEMU workflow, also run:

- `cargo run -p xtask -- test-qemu`

If your change affects the controlled exception path, also run:

- `cargo run -p xtask -- test-exception`

If you want the standard combined local validation flow, run:

- `cargo run -p xtask -- test`

## License

MIT

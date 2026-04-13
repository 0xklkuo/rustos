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
- Milestone 0 — workspace and project foundation

## Roadmap

See:
- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/decisions/0001-target-platform.md`

## Local Development

Planned local workflow will center around:

- Rust toolchain
- QEMU
- UEFI-based boot flow
- `cargo xtask` commands for common tasks

Detailed setup instructions will be added as the bootable kernel milestone is implemented.

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

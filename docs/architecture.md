# Architecture

## Purpose

`rustos` is a minimal, educational, and maintainable operating system project written in Rust.

The project is designed to help developers learn:
- core Rust systems programming patterns
- modern operating system fundamentals
- practical open-source engineering habits

The architecture favors clarity over completeness. Every subsystem should be small, explicit, and easy to reason about.

## Goals

- Keep the codebase minimal and understandable
- Build a bootable kernel foundation with clear module boundaries
- Use modern tooling and workflows that are easy to reproduce
- Make the project approachable for contributors with different experience levels
- Document key decisions early to reduce confusion and scope drift

## Non-Goals for MVP

The MVP is not intended to provide:
- a full Unix-compatible environment
- multitasking
- a filesystem
- networking
- user-space applications
- broad hardware support
- multi-architecture support

These may be explored later, but they are intentionally out of scope for the first milestones.

## Target Platform

### MVP Target
- `x86_64-unknown-uefi`

### Host Development Environment
- Apple Silicon macOS host
- QEMU used to run the target image through emulation

### Why This Target

This target is chosen for educational clarity and ecosystem support.

Benefits:
- strong alignment with existing Rust OS learning material
- simpler early-stage debugging and documentation
- easier contributor onboarding
- clear UEFI-first boot path
- practical to run from Apple Silicon using QEMU

### Deferred target support

Support for `aarch64` may be considered later after the MVP foundation is stable.

## High-Level Repository Structure

The project uses a monorepo with a small Rust workspace.

- `kernel/` — kernel crate and low-level OS code
- `xtask/` — developer workflow commands implemented in Rust
- `docs/` — concise project documentation and architecture decisions
- `.github/` — CI and contribution templates

This structure keeps code, tooling, and documentation close together while avoiding unnecessary fragmentation.

## Architectural Principles

### 1. Minimal first
Start with the smallest working system. Add complexity only when it is justified by a clear learning or engineering need.

### 2. Explicit boundaries
Keep responsibilities separated by module and crate. Avoid hidden behavior and avoid abstractions that make control flow harder to understand.

### 3. Unsafe is isolated
Unsafe code should be:
- minimized
- documented
- reviewed carefully
- kept close to the hardware or ABI boundary it serves

Each unsafe block should have a short explanation of its safety assumptions when the code becomes non-trivial.

### 4. Code-first, docs-supported
The code should remain the primary source of truth. Documentation should explain intent, constraints, and decisions rather than restating implementation details.

### 5. Reproducible workflows
Common tasks should be easy to run and easy to discover. Developer workflows should be scripted and versioned with the repository.

### 6. Educational clarity over cleverness
Prefer straightforward code over compact or overly abstract solutions. The project should teach by being readable.

## Planned System Shape

The early system is expected to grow in small stages.

### Stage 0
Repository and workspace foundation:
- workspace setup
- documentation
- contribution baseline
- CI baseline

### Stage 1
Minimal bootable kernel:
- UEFI entry path
- basic output
- panic handling
- emulator run workflow

### Stage 2
Kernel structure for growth:
- architecture-specific module boundaries
- console and logging support
- boot and panic support
- placeholders for memory and interrupts

### Stage 3
Core runtime groundwork:
- interrupt setup
- timer support
- memory initialization
- allocator planning

## Kernel Design Direction

The kernel should remain small and modular.

Expected early module boundaries:
- `arch` — architecture-specific code
- `boot` — boot and entry-related logic
- `console` — early output and logging
- `panic` — panic reporting and halt behavior
- `memory` — memory initialization and allocation groundwork

These boundaries are intended to keep the code understandable as the project grows.

## Boot Strategy

The project will use a UEFI-first boot path for the MVP.

Reasons:
- modern and cleaner than legacy BIOS-first approaches
- easier to explain as a current baseline
- aligns with the chosen target and educational goals

The exact boot implementation should remain as small as possible in early milestones.

## Tooling Strategy

The project should use a minimal modern Rust workflow.

Planned tooling:
- Rust workspace
- pinned toolchain configuration
- `cargo fmt`
- `cargo clippy`
- `cargo check`
- `cargo xtask` for project-specific workflows
- GitHub Actions for CI
- QEMU for local execution

Tooling should support the project without becoming a project of its own.

## Documentation Strategy

Documentation should be concise and practical.

Primary documentation types:
- `README.md` for project overview and getting started
- `docs/roadmap.md` for milestone planning and scope
- `docs/architecture.md` for system shape and principles
- `docs/decisions/` for architecture decision records

Documentation should answer:
- what the project is
- why a decision was made
- what is currently in scope
- what is intentionally deferred

## Contribution and Maintenance Principles

To keep the project maintainable:
- prefer small pull requests
- keep module responsibilities narrow
- avoid introducing abstractions before they are needed
- document decisions that affect future contributors
- keep examples and commands current
- treat readability as a feature

## Decision Review Policy

Architecture decisions should be revisited when:
- the MVP is complete
- a major subsystem requires a different boundary
- contributor friction reveals a workflow problem
- target platform assumptions change

Until then, the project should resist unnecessary expansion.

## Summary

`rustos` is intentionally small in scope and deliberate in structure.

The architecture is designed to support:
- a clean educational path
- a minimal bootable kernel
- a maintainable open-source repository
- future growth without early over-engineering

The project should remain simple enough to learn from and strong enough to build on.

# Architecture

## Purpose

`rustos` is a minimal, educational, and maintainable operating system project written in Rust.

This document explains the stable architectural shape of the project:

- what the system is trying to be
- which boundaries matter
- how the repository is organized
- which principles should guide future changes

This document is intentionally not the project status page.

For milestone status, implementation progress, and release sequencing, see `docs/roadmap.md`.

## Project Shape

`rustos` is a small Rust workspace for learning modern operating system fundamentals through a real codebase.

The project is built around a few clear layers:

- `kernel/` for firmware-facing and low-level OS code
- `nucleus/` for host-testable pure logic shared with the kernel
- `xtask/` for developer workflows
- `docs/` for concise design notes and decisions
- `.github/` for CI and contribution templates

The architecture should remain small enough that a new contributor can understand the main structure quickly.

## Goals

The architecture should support these goals:

- keep the codebase minimal and understandable
- make subsystem boundaries explicit
- keep low-level code teachable
- support reproducible local development and CI
- make host-testable logic easy to isolate
- help contributors understand what is implemented and what is deferred

## Non-Goals for the MVP

The MVP architecture is not intended to support:

- a full Unix-compatible environment
- multitasking
- a filesystem implementation
- networking
- user-space applications
- broad hardware support
- multi-architecture support
- large framework-style abstractions

These may be explored later, but they should not shape the early architecture prematurely.

## Target Platform

### Current target

The current target platform is:

- `x86_64-unknown-uefi`

The current host development assumption is:

- Apple Silicon macOS host
- QEMU for emulation

### Why this target

This target is chosen because it keeps the early project practical and teachable:

- it aligns well with existing Rust OS learning material
- it keeps the boot path modern and explicit
- it is practical to run through QEMU from Apple Silicon
- it avoids introducing multiple target concerns too early

### Deferred target support

Support for other targets, including `aarch64`, is intentionally deferred until the current foundation is stable.

The rationale for the current target choice is recorded in:

- `docs/decisions/0001-target-platform.md`

## Architectural Principles

### 1. Minimal first

Start with the smallest useful system.

Add complexity only when it is justified by:

- a clear learning benefit
- a real implementation need
- a clearer subsystem boundary

### 2. Explicit boundaries

The project should prefer visible boundaries over hidden behavior.

That applies to:

- crate boundaries
- module boundaries
- firmware and architecture boundaries
- host-testable versus runtime-only logic
- implemented versus deferred subsystem behavior

### 3. Unsafe is isolated

Unsafe code should stay close to the boundary that requires it.

In practice, that means:

- keep unsafe code near hardware, ABI, or firmware interaction
- avoid spreading unsafe assumptions through higher-level logic
- document invariants when unsafe behavior becomes non-trivial

### 4. Host-testable logic first

If logic can be tested on the host, it should prefer living in `nucleus/`.

This keeps:

- unit tests fast
- logic easier to reason about
- firmware-facing code smaller
- emulator-only validation focused on real runtime behavior

### 5. Code-first, docs-supported

Code is the primary source of truth.

Documentation should explain:

- intent
- constraints
- boundaries
- tradeoffs

Documentation should avoid repeating implementation details that are already obvious from the code.

### 6. Reproducible workflows

Common development tasks should be:

- explicit
- versioned
- easy to discover
- aligned between local development and CI

The project should prefer simple workflow entry points over hidden tooling layers.

### 7. Educational clarity over cleverness

The architecture should remain readable.

Prefer:

- straightforward control flow
- small modules
- simple names
- explicit state
- narrow interfaces

Avoid abstractions that make the system harder to learn from.

## Repository Structure

The repository uses a small Rust workspace.

### `kernel/`

The `kernel/` crate contains:

- the UEFI entry path
- boot sequencing
- architecture-facing runtime code
- console output
- interrupt and exception runtime boundaries
- memory and paging runtime boundaries
- panic handling
- syscall-facing kernel boundary code

This crate should remain small and focused on runtime-facing behavior.

### `nucleus/`

The `nucleus/` crate contains host-testable pure logic.

Examples include:

- runtime state summaries
- interrupt state models
- memory bookkeeping helpers
- paging helpers
- syscall models
- task models
- descriptor models

This crate exists to keep pure logic separate from firmware-facing runtime code.

### `xtask/`

The `xtask/` crate contains developer workflow commands.

It is the main entry point for:

- checking
- formatting
- linting
- unit tests
- QEMU smoke tests
- exception smoke tests
- local run workflows

This keeps project workflows explicit and Rust-native.

### `docs/`

The `docs/` directory contains concise project documentation.

It should contain a small number of document types:

- architecture and roadmap documents
- testing and workflow guidance
- subsystem direction notes
- architecture decision records

### `.github/`

The `.github/` directory contains:

- CI workflows
- issue templates
- pull request templates

This supports contributor experience and project hygiene.

## System Boundaries

The architecture is intentionally organized around a few stable boundaries.

### Boot boundary

The boot boundary is responsible for:

- entering through UEFI
- selecting the current boot mode
- initializing early runtime subsystems in a visible order
- keeping boot behavior easy to inspect

The boot path should remain explicit and small.

### Architecture boundary

The architecture boundary isolates target-specific behavior.

It should contain:

- architecture-specific low-level setup
- exception and interrupt hooks
- paging-facing architecture hooks
- CPU-specific runtime helpers

Shared logic should stay outside this boundary whenever possible.

### Runtime subsystem boundaries

The kernel is organized into small subsystem boundaries such as:

- `console`
- `interrupt`
- `memory`
- `paging`
- `panic`
- `syscall`

These boundaries should remain narrow and easy to understand.

They do not need to be complete subsystems early. They only need to make responsibilities explicit.

### Host-testable logic boundary

The `nucleus` boundary exists so the project can separate:

- pure logic
- state models
- summary helpers
- small validation rules

from:

- firmware interaction
- architecture-specific runtime behavior
- emulator-only behavior

This is one of the most important architectural choices in the project.

## Kernel Design Direction

The kernel should remain modular and small.

The current architectural direction favors:

- explicit initialization order
- plain-language runtime logs
- narrow subsystem entry points
- minimal public surfaces
- small state types over large frameworks

The kernel should grow by adding small justified boundaries, not by introducing large generic infrastructure.

## Unix-like Direction

`rustos` aims toward a small Unix-like teaching kernel.

At the architectural level, that means the project should gradually move toward:

- a clear kernel and user boundary
- a syscall-oriented service boundary
- a task-oriented execution model
- descriptor-like resource references
- a later VFS boundary

This direction should remain incremental.

The architecture should not assume:

- full POSIX compatibility
- early user-space support
- a large syscall surface
- a complete process model
- a complete VFS

The umbrella direction is described in:

- `docs/unix-like.md`

Subsystem-specific direction is described in:

- `docs/paging.md`
- `docs/syscalls.md`
- `docs/tasks.md`
- `docs/descriptors.md`

## Boot Strategy

The project uses a UEFI-first boot path for the current stage.

The early system is intentionally simple:

- a small UEFI application entry path
- direct QEMU execution
- explicit firmware configuration
- deterministic boot output

This keeps the boot model understandable and easy to debug.

A more complex loader split can be considered later only if the project clearly needs it.

## Tooling Strategy

The project uses a minimal Rust-native workflow.

Core tooling includes:

- Rust workspace support
- nightly Rust where required by low-level boundaries
- `cargo fmt`
- `cargo clippy`
- `cargo check`
- `cargo xtask`
- GitHub Actions
- QEMU for runtime validation

Tooling should support the project without becoming a subsystem of its own.

## Documentation Strategy

The documentation set should have clear ownership.

### `README.md`
Use for:

- project overview
- quick start
- current capabilities at a high level
- links to deeper docs

### `docs/architecture.md`
Use for:

- stable architectural principles
- repository structure
- system boundaries
- long-lived design direction

### `docs/roadmap.md`
Use for:

- milestone status
- implementation sequencing
- release shape
- success metrics

### subsystem docs
Use for:

- subsystem-specific design direction
- minimal models
- deferred items
- relationships to other boundaries

### decision records
Use for:

- durable architectural decisions
- rationale that should not be repeated everywhere else

## Contribution and Maintenance Principles

To keep the architecture maintainable:

- prefer small pull requests
- keep responsibilities narrow
- avoid speculative abstractions
- document durable decisions early
- keep workflows and docs current
- treat readability as a feature

## Decision Review Policy

Architecture decisions should be revisited when:

- the target platform changes
- the boot strategy changes
- the host development assumptions change
- the MVP scope changes significantly
- a subsystem clearly outgrows its current boundary

Until then, the project should resist unnecessary expansion.

## Summary

`rustos` is intentionally small in scope and deliberate in structure.

Its architecture is designed to support:

- a clean educational path
- a minimal bootable kernel
- a maintainable open-source repository
- future growth without early over-engineering

The project should remain simple enough to learn from and strong enough to build on.

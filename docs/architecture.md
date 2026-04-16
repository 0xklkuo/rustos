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

## Unix-like Direction

`rustos` aims to grow toward a small Unix-like teaching kernel, but this direction should stay explicit and intentionally incomplete during the MVP.

For this project, "Unix-like" currently means:
- a clear kernel and user boundary
- a small syscall-oriented execution model
- a simple task or process direction
- a filesystem abstraction boundary
- plain-language documentation of what is implemented versus deferred

For the current milestones, this direction is a design guide, not a promise of feature completeness.

The project should avoid:
- claiming POSIX compatibility too early
- introducing a large syscall surface before the runtime and memory foundations are stable
- designing a full VFS before there is a concrete need for one
- adding user-space support before the kernel boundary is documented clearly

A dedicated Unix-like direction note should define:
- syscall direction
- task and process model direction
- VFS direction
- user and kernel boundary expectations
- explicit non-goals for the MVP

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
Minimal bootable UEFI application:
- UEFI application entry path
- basic console output
- panic handling through UEFI helpers
- direct QEMU boot workflow with a small EFI disk image

### Stage 2
Kernel structure for growth:
- architecture-specific module boundaries
- console and logging support
- boot and panic support
- placeholders for memory and interrupts
- a minimal module layout for `arch`, `boot`, `console`, `panic`, and `memory`

### Stage 3
Core runtime groundwork:
- explicit runtime initialization order
- exception groundwork
- interrupt setup skeleton
- timer groundwork
- idle or halt behavior placeholder
- memory initialization planning
- allocator planning

### Stage 4
Memory management foundation:
- memory map handling
- frame allocator skeleton
- explicit memory subsystem state
- heap strategy decision
- minimal allocator only if justified

See `docs/roadmap.md` for the implementation sequence and milestone planning for the Unix-like MVP track.


## Kernel Design Direction

The kernel should remain small and modular.

Expected early module boundaries:
- `arch` — architecture-specific code and target-specific setup
- `boot` — boot entry flow and early initialization order
- `console` — early output and logging helpers
- `interrupt` — exception and hardware interrupt groundwork
- `panic` — panic reporting and panic-related support
- `memory` — memory initialization and allocation groundwork

These boundaries are intended to keep the code understandable as the project grows.

For Milestone 3, these modules should remain intentionally small. The goal is to make responsibilities obvious without introducing deep abstraction, generic frameworks, or speculative subsystem design.

For Milestone 4, the same modules should begin to expose a clearer runtime sequence without becoming a full subsystem implementation. The immediate goal is to make initialization order visible and easy to reason about before introducing real interrupt tables, timer drivers, or memory managers.

For the next exception and interrupt groundwork stage, the interrupt module should become more explicit without pretending to be complete. The immediate goal is to introduce a small, well-documented subsystem shape that can answer:
- whether exception groundwork has been initialized
- whether breakpoint and double-fault handling are planned and visible
- whether timer interrupt groundwork exists
- which interrupt paths are intentionally deferred
- how success and failure should appear in logs and bounded emulator tests

This stage should still avoid premature IDT, PIC, APIC, or keyboard-driver complexity that is not yet justified by the code.

For Milestone 5, the memory module should become more explicit without pretending to be complete. The immediate goal is to introduce a small, well-documented memory subsystem shape that can answer:
- whether memory initialization has happened
- what memory information is currently known
- where a future frame allocator will live
- whether heap support exists yet

This stage should still avoid premature paging abstractions, allocator frameworks, or architecture-general memory models that are not yet justified by the code.

## Boot Strategy

The project will use a UEFI-first boot path for the MVP.

In the first bootable milestone, `rustos` is implemented as a small UEFI application rather than a fully separated bootloader-plus-kernel design.

Reasons:
- it is the smallest practical path to a bootable artifact
- it keeps the entry flow easy to read and debug
- it allows early output and panic handling with minimal setup
- it avoids introducing a second binary and handoff boundary too early

Reasons for choosing UEFI first:
- modern and cleaner than legacy BIOS-first approaches
- easier to explain as a current baseline
- aligns with the chosen target and educational goals

For local execution, the project should prefer a direct QEMU workflow over a higher-level runner abstraction.

Reasons:
- it keeps the boot path explicit and easier to debug
- it avoids hidden assumptions about firmware naming and host packaging
- it reduces dependency on extra tooling that is not essential to the project
- it is easier to document and maintain across environments
- it better matches the educational goal of showing how the system is actually booted

The direct run flow should stay minimal:
- build the UEFI binary
- place it at the default removable-media boot path
- create a small EFI disk image
- launch QEMU with explicit firmware and disk arguments

The exact boot implementation should remain as small as possible in early milestones. A more explicit loader and kernel split can be introduced later if the project outgrows the single-image approach.

As the kernel structure is introduced, boot behavior should stay unchanged while code is moved behind clearer module boundaries. Structural refactors should preserve the current boot path and deterministic output.

As runtime groundwork is added, the boot path should log each initialization phase in plain language. Early milestones should prefer visible sequencing such as:
- console initialization
- architecture initialization
- exception groundwork
- interrupt groundwork
- timer groundwork
- memory groundwork
- transition to idle or halt behavior

This keeps the system teachable and makes failures easier to localize during early boot.

As exception and interrupt groundwork is added, the project should keep the subsystem narrow and explicit. Early interrupt-related code should prefer visible sequencing such as:
- exception subsystem state creation
- breakpoint and double-fault groundwork reporting
- hardware interrupt groundwork reporting
- timer interrupt readiness reporting
- clear separation between implemented paths and deferred paths

This keeps low-level runtime work understandable and avoids introducing a large interrupt framework before the project has a concrete need for it.

As memory groundwork is added, the memory path should remain equally explicit. Early memory code should prefer visible sequencing such as:
- memory subsystem state creation
- memory map placeholder or discovery boundary
- frame allocator placeholder
- heap support status
- clear reporting of what is initialized versus deferred

This keeps the memory subsystem understandable and prevents the project from drifting into opaque low-level setup too early.

As the Unix-like direction becomes more concrete, it should be layered on top of these foundations rather than introduced in parallel as a large design exercise. In practice, that means:
- syscall direction should follow the runtime and memory groundwork
- task and process direction should follow clearer execution and ownership boundaries
- VFS direction should follow a concrete storage and memory story
- user-space direction should remain deferred until the kernel boundary is explicit and teachable

This keeps the Unix-like direction realistic, incremental, and aligned with the project's educational goals.

## Tooling Strategy

The project should use a minimal modern Rust workflow.

Planned tooling:
- Rust workspace
- stable Rust toolchain with the `x86_64-unknown-uefi` target
- `cargo fmt`
- `cargo clippy`
- `cargo check`
- `cargo xtask` for project-specific workflows
- GitHub Actions for CI
- direct QEMU-based local execution through a small EFI directory workflow

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

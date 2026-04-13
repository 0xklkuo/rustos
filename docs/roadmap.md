# rustos Roadmap

## Purpose

`rustos` is a minimal, educational, and maintainable open-source project for learning the Rust ecosystem and modern operating system fundamentals by building a small OS from scratch.

The project prioritizes:

- minimalism over feature count
- clarity over cleverness
- explicit design over hidden magic
- contributor understanding over premature optimization

## MVP Definition

The MVP is a small, bootable kernel foundation that:

- targets `x86_64-unknown-uefi`
- runs in QEMU from an Apple Silicon Mac host
- boots reliably with a documented local workflow
- prints deterministic boot output
- defines a panic path
- has a clean monorepo structure
- includes concise contributor and architecture documentation
- includes basic CI for formatting and build validation

## Non-Goals for MVP

The following are intentionally out of scope for the MVP:

- multitasking
- user-space programs
- filesystem support
- networking
- shell or command interpreter
- multi-architecture support
- Docker-first development
- hardware support beyond the emulator target
- advanced memory management beyond foundational setup

## Roadmap Principles

Each milestone should:

- keep the codebase small and readable
- introduce only one layer of complexity at a time
- document important decisions early
- isolate `unsafe` code and explain its invariants
- prefer stable workflows and simple tooling
- avoid abstractions that do not clearly improve learning value

## Milestones

### Milestone 0 — Foundation and Scope Lock

Goal: create a clean project foundation before kernel work begins.

Deliverables:

- Cargo workspace scaffold
- toolchain definition
- README
- MIT license
- contribution guide
- architecture note
- roadmap
- initial architecture decision record
- issue and pull request templates
- placeholder `kernel` and `xtask` crates

Acceptance criteria:

- the repository structure is clear
- a new contributor can understand the project purpose quickly
- the workspace builds for host-side tooling
- project scope and non-goals are documented

### Milestone 1 — Minimal Bootable UEFI Kernel

Goal: produce the smallest bootable kernel artifact.

Deliverables:

- `no_std` kernel entry
- panic handler
- basic output path
- QEMU run workflow
- `xtask run`
- concise local setup instructions

Acceptance criteria:

- the project builds on an Apple Silicon host
- QEMU launches the image successfully
- the kernel prints a deterministic boot message
- panic behavior is defined and understandable

### Milestone 2 — Developer Workflow and CI

Goal: make the project easy to run, check, and contribute to.

Deliverables:

- `xtask fmt`
- `xtask lint`
- `xtask check`
- CI workflow
- contributor-facing templates and guidance

Acceptance criteria:

- local checks are easy to run
- CI validates formatting and build steps
- contributor instructions remain short and accurate

### Milestone 3 — Kernel Structure for Growth

Goal: organize the kernel into clear, minimal modules.

Deliverables:

- initial module layout for:
  - `arch`
  - `boot`
  - `console`
  - `panic`
  - `memory`
- explicit unsafe boundaries
- architecture-specific code isolated from shared logic

Acceptance criteria:

- module responsibilities are obvious
- unsafe code is documented with invariants
- the structure supports future growth without over-engineering

### Milestone 4 — Interrupt and Time Groundwork

Goal: prepare the kernel for basic runtime behavior.

Deliverables:

- interrupt setup skeleton
- timer groundwork
- idle or halt loop
- improved boot and initialization logging

Acceptance criteria:

- initialization order is visible in logs
- low-level runtime setup is easier to reason about
- failures are easier to diagnose

### Milestone 5 — Memory Management Foundation

Goal: introduce the smallest useful memory subsystem.

Deliverables:

- memory map handling
- frame allocator skeleton
- heap strategy decision
- minimal allocator only if justified

Acceptance criteria:

- memory initialization is documented
- the design remains intentionally minimal
- no unnecessary generalization is introduced

### Milestone 6 — Unix-like Direction Definition

Goal: define the first Unix-like boundaries without pretending to be complete.

Deliverables:

- syscall direction note
- task or process model sketch
- VFS direction note
- user and kernel boundary plan

Acceptance criteria:

- the meaning of "Unix-like" is clearly defined for this project
- non-goals remain explicit
- the roadmap stays realistic and educational

## Suggested Release Shape

A practical early release sequence:

- `v0.1.0`: repository foundation and documentation
- `v0.2.0`: bootable UEFI kernel in QEMU
- `v0.3.0`: developer workflow and CI baseline
- `v0.4.0`: kernel structure and low-level groundwork
- `v0.5.0`: memory foundation and Unix-like direction notes

## Success Metrics

The project is on track if:

- a new contributor can understand the repo layout quickly
- local setup is short and reproducible
- each milestone leaves the codebase cleaner, not more confusing
- documentation stays concise and accurate
- the project remains small enough to study comfortably

## Review Policy

This roadmap should be revisited when one of the following changes:

- the target architecture changes
- the boot strategy changes
- the host development assumptions change
- the MVP scope expands
- the educational goals shift significantly

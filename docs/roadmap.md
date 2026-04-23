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
- uses a direct QEMU workflow instead of a runner-specific wrapper

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

Current implementation status:

- the Cargo workspace, toolchain definition, README, license, contribution guide, architecture note, roadmap, and issue templates are present
- the repository structure is clear and contributor-facing project scope is documented
- the workspace builds for host-side tooling
- this milestone is effectively complete for the current project stage

Acceptance criteria:

- the repository structure is clear
- a new contributor can understand the project purpose quickly
- the workspace builds for host-side tooling
- project scope and non-goals are documented

### Milestone 1 — Minimal Bootable UEFI Kernel

Goal: produce the smallest bootable kernel artifact with the simplest practical local workflow.

Deliverables:

- `no_std` kernel entry
- panic handler
- basic output path
- direct QEMU run workflow
- `xtask run`
- concise local setup instructions
- minimal EFI disk image creation

Current implementation status:

- a minimal UEFI application entry path has been introduced
- deterministic boot output is implemented
- `xtask` commands now cover `check`, `fmt`, `lint`, and `run`
- CI now includes a UEFI target build step
- the direct QEMU workflow is implemented and documented
- this milestone is effectively complete for the current project stage

Acceptance criteria:

- the project builds on an Apple Silicon host
- QEMU launches the image successfully
- the kernel prints a deterministic boot message
- panic behavior is defined and understandable
- the local run path is explicit and easy to debug

### Milestone 2 — Developer Workflow and CI

Goal: make the project easy to run, check, and contribute to.

Deliverables:

- `xtask fmt`
- `xtask lint`
- `xtask check`
- CI workflow
- contributor-facing templates and guidance

Current implementation status:

- `xtask` already provides `check`, `fmt`, `lint`, `run`, `test-unit`, `test-qemu`, `test-exception`, and `test`
- the local workflow uses direct QEMU execution instead of a wrapper tool
- CI already validates workspace checks, formatting, clippy, unit tests, the bounded QEMU boot smoke test, and a separate exception smoke test
- contributor-facing templates and baseline guidance are already present
- this milestone is effectively complete for the current project stage

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

Current implementation status:

- the kernel now has explicit `arch`, `boot`, `console`, `interrupt`, `panic`, and `memory` modules
- the current structure keeps firmware-facing code small while allowing host-testable logic to live in `nucleus`
- the existing `xtask` workflow and bounded QEMU test provide a safe validation path for further structural refinement
- this milestone is effectively complete for the current project stage

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

Current implementation status:

- the kernel now has a minimal module layout that can support runtime initialization growth
- the boot flow now exposes a small explicit initialization sequence for architecture, interrupts, timer, memory, panic, and idle behavior
- boot logs now show initialization order without changing the current boot success path
- the first real breakpoint-handler path is now implemented and validated through bounded QEMU testing
- interrupt and timer support still remain intentionally minimal beyond the breakpoint-first exception path
- exception, interrupt, and timer readiness logs should continue to distinguish modeled groundwork from installed low-level handlers
- this milestone remains partially complete because timer and broader interrupt runtime setup are still deferred

Acceptance criteria:

- initialization order is visible in logs
- low-level runtime setup is easier to reason about
- failures are easier to diagnose

### Milestone 5 — Memory Management Foundation

Status: in progress

Goal: introduce the smallest useful memory subsystem.

Deliverables:

- memory map handling
- frame allocator skeleton
- heap strategy decision
- minimal allocator only if justified

Current implementation status:

- the current memory module exposes a small initialization state and a frame allocator skeleton through host-testable logic in `nucleus`
- the project now also has a small host-testable discovered-memory summary type for descriptor counts, conventional-memory region counts, and conventional-memory bytes
- the kernel memory path now reads real discovered memory information from the UEFI memory map at runtime
- the project is beginning to connect the frame allocator direction to discovered conventional memory through a minimal frame allocator seed
- memory-related boot output should stay explicit and plain-language so contributors can see what is initialized, what is discovered, and what is still deferred
- allocator work should remain a documented decision first, and only become code if it is clearly needed by the next milestone

Acceptance criteria:

- memory initialization is documented
- the design remains intentionally minimal
- no unnecessary generalization is introduced

### Milestone 6 — Unix-like Direction Definition

Status: documentation-complete

Goal: define the first Unix-like boundaries without pretending to be complete.

Deliverables:

- syscall direction note
- task or process model sketch
- VFS direction note
- user and kernel boundary plan

Current implementation status:

- the project now has a minimal bootable kernel foundation, a clear module layout, and explicit runtime logging
- the project now also has a dedicated Unix-like direction note that defines syscall, task, VFS, and user/kernel boundary goals
- the Unix-like direction remains documentation-first and does not imply full Unix compatibility
- this milestone now gives future subsystem work clearer goals and non-goals

Acceptance criteria:

- the meaning of "Unix-like" is clearly defined for this project
- non-goals remain explicit
- the roadmap stays realistic and educational

## Unix-like MVP Implementation Phases

These U-series phases extend the earlier foundation milestones and provide the implementation order for the Unix-like MVP. They should stay aligned with:

- `docs/blog-os-adoption.md`
- `docs/testing.md`
- `docs/unix-like.md`

The purpose of these phases is to keep the next work explicit, incremental, and easy to evaluate against the project's minimal and educational goals.

### U1 — Testing Foundation and `blog_os` Adoption Plan

Goal: establish the testing and reference policy needed for deeper kernel work.

Deliverables:

- testing strategy
- `blog_os` adoption policy
- split host-side unit tests from bounded QEMU tests
- keep local and CI validation aligned around explicit workflow commands

Current implementation status:

- the project now has a documented testing strategy
- the project now has a documented `blog_os` adoption policy
- the local and CI workflows now distinguish host-side unit tests from bounded QEMU tests
- the project now has a host-testable `nucleus` crate for pure logic and unit tests

Acceptance criteria:

- the testing strategy is documented clearly
- the `blog_os` adoption policy is documented clearly
- host-side unit tests and bounded QEMU tests are separate and explicit
- local and CI validation use the same workflow shape

### U2 — Exception and Interrupt Groundwork

Status: in progress

Goal: introduce the smallest useful exception and interrupt foundation aligned with the current minimal kernel direction.

Deliverables:

- exception handling direction
- breakpoint and double-fault groundwork
- hardware interrupt groundwork
- timer-first interrupt direction
- host-testable interrupt state
- bounded QEMU validation for interrupt-related runtime behavior

Current implementation status:

- the project now has explicit exception and interrupt subsystem state instead of treating interrupts as part of generic architecture state
- the boot flow now reports exception and interrupt initialization separately in plain language
- exception and interrupt work still stays minimal and avoids full PIC or APIC complexity
- host-side unit tests now cover the new pure interrupt logic in `nucleus`
- the project now includes a real breakpoint-handler path, while broader hardware interrupt handling remains deferred

Acceptance criteria:

- exception and interrupt groundwork are represented explicitly in code
- host-side unit tests cover the new pure logic
- boot logs show exception and interrupt initialization separately
- bounded QEMU validation still passes
- the implementation stays small and educational

### U3 — Controlled Exception Path

Status: complete for the first breakpoint path

Goal: introduce the first narrow, testable exception path.

Deliverables:

- one controlled exception path
- bounded emulator validation for the chosen exception
- explicit success and failure reporting
- clearer direction for later double-fault handling

Current implementation status:

- the project now has explicit exception groundwork and host-testable interrupt state in `nucleus`
- explicit exception-test boot mode selection is part of the project workflow
- the controlled exception path now includes a real breakpoint-first exception path
- the exception path now produces a clear success marker in bounded QEMU output only when the real handler path is reached
- double-fault handling remains deferred until the first controlled exception path is stable and easy to validate

Acceptance criteria:

- an explicit exception-test boot mode exists and is easy to trigger from the project workflow
- one real exception path is implemented and visible
- the exception path is validated through bounded QEMU testing
- the success marker is emitted by the real handler path, not by ordinary post-trigger control flow
- the implementation remains narrow and easy to reason about

### U4 — Real Memory Foundation

Status: in progress

Goal: move from placeholder memory state to real discovered memory information.

Deliverables:

- real memory map boundary
- frame allocator groundwork backed by discovered memory
- host-side tests for memory bookkeeping where possible

Current implementation status:

- the project now has a small host-testable discovered-memory summary in `nucleus`
- the kernel memory path now connects that summary to real UEFI memory-map discovery
- the frame allocator direction is beginning to connect to discovered conventional memory through a minimal seed, without expanding into paging or heap work

Acceptance criteria:

- memory work is backed by real discovered memory information
- the frame allocator direction is connected to actual memory information
- the implementation remains explicit and minimal

### U5 — Paging and Heap Direction

Status: documentation-only

Goal: define the smallest useful paging and heap direction.

Deliverables:

- paging direction notes
- minimal paging groundwork
- explicit heap strategy decision
- allocator work only if justified by the code

Acceptance criteria:

- paging direction is documented clearly
- paging groundwork is introduced only as needed
- heap support remains deferred unless justified by a concrete need

### U6 — Unix-like Kernel Boundary

Status: documentation-only

Goal: make the Unix-like direction concrete through small kernel interfaces.

Deliverables:

- syscall boundary notes
- task model sketch
- descriptor or handle direction
- VFS direction
- clearer user and kernel boundary planning

Acceptance criteria:

- the Unix-like direction is reflected in small explicit kernel boundaries
- syscall, task, and VFS direction are documented clearly
- the implementation order remains realistic and educational

## Suggested Release Shape

A practical early release sequence:

- `v0.1.0`: repository foundation and documentation
- `v0.2.0`: bootable UEFI kernel in QEMU
- `v0.3.0`: developer workflow and CI baseline
- `v0.4.0`: kernel structure and low-level groundwork
- `v0.5.0`: memory foundation and Unix-like direction notes
- `v0.6.0`: testing foundation and exception/interrupt groundwork

## Success Metrics

The project is on track if:

- a new contributor can understand the repo layout quickly
- local setup is short and reproducible
- each milestone leaves the codebase cleaner, not more confusing
- documentation stays concise and accurate
- the project remains small enough to study comfortably
- local boot workflows avoid unnecessary wrapper tools and hidden assumptions

## Review Policy

This roadmap should be revisited when one of the following changes:

- the target architecture changes
- the boot strategy changes
- the host development assumptions change
- the MVP scope expands
- the educational goals shift significantly

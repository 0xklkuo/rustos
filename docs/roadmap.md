# rustos Roadmap

## Purpose

This document is the single source of truth for project status, milestone sequencing, and release direction in `rustos`.

Use this document to answer:

- what the project is trying to achieve
- what is in scope for the MVP
- what is intentionally deferred
- which milestone is current
- what has already been completed
- what should happen next

Other documents should explain architecture, subsystem direction, testing, and decisions. They should not duplicate project status.

## Project Summary

`rustos` is a minimal, educational, and maintainable open-source project for learning Rust systems programming and modern operating system fundamentals by building a small OS from scratch.

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
- POSIX compatibility claims

## Roadmap Principles

Each milestone should:

- keep the codebase small and readable
- introduce only one layer of complexity at a time
- document important decisions early
- isolate `unsafe` code and explain its invariants
- prefer stable workflows and simple tooling
- avoid abstractions that do not clearly improve learning value

## Status Model

This roadmap uses two layers:

- **Foundation milestones** (`Milestone 0` through `Milestone 6`)
  - broad project stages
- **Unix-like implementation phases** (`U1` through `U6`)
  - narrower implementation phases layered on top of the foundation

Use the foundation milestones to understand the overall project shape.
Use the U-series phases to understand the current implementation track.

## Current Status

### Current implementation track
- `U6 — Unix-like Kernel Boundary`

### Current release preparation target
- `v0.1.0-alpha.1`

### Current project state
The project currently provides:

- a bootable UEFI kernel foundation
- a direct QEMU workflow
- split host-side and emulator validation
- a real breakpoint-first exception path
- real UEFI memory-map discovery
- a minimal paging direction boundary
- a minimal Unix-like boundary starter for:
  - syscalls
  - tasks
  - descriptor-like handles

The project is still intentionally early-stage and educational.

## Foundation Milestones

### Milestone 0 — Foundation and Scope Lock

**Goal:** create a clean project foundation before kernel work begins.

**Deliverables:**

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

**Status:** complete

**Acceptance criteria:**

- the repository structure is clear
- a new contributor can understand the project purpose quickly
- the workspace builds for host-side tooling
- project scope and non-goals are documented

---

### Milestone 1 — Minimal Bootable UEFI Kernel

**Goal:** produce the smallest bootable kernel artifact with the simplest practical local workflow.

**Deliverables:**

- `no_std` kernel entry
- panic handler
- basic output path
- direct QEMU run workflow
- `xtask run`
- concise local setup instructions
- minimal EFI disk image creation

**Status:** complete

**Acceptance criteria:**

- the project builds on an Apple Silicon host
- QEMU launches the image successfully
- the kernel prints a deterministic boot message
- panic behavior is defined and understandable
- the local run path is explicit and easy to debug

---

### Milestone 2 — Developer Workflow and CI

**Goal:** make the project easy to run, check, and contribute to.

**Deliverables:**

- `xtask fmt`
- `xtask lint`
- `xtask check`
- CI workflow
- contributor-facing templates and guidance

**Status:** complete

**Acceptance criteria:**

- local checks are easy to run
- CI validates formatting and build steps
- contributor instructions remain short and accurate

---

### Milestone 3 — Kernel Structure for Growth

**Goal:** organize the kernel into clear, minimal modules.

**Deliverables:**

- initial module layout for:
  - `arch`
  - `boot`
  - `console`
  - `panic`
  - `memory`
- explicit unsafe boundaries
- architecture-specific code isolated from shared logic

**Status:** complete

**Acceptance criteria:**

- module responsibilities are obvious
- unsafe code is documented with invariants
- the structure supports future growth without over-engineering

---

### Milestone 4 — Interrupt and Time Groundwork

**Goal:** prepare the kernel for basic runtime behavior.

**Deliverables:**

- interrupt setup skeleton
- timer groundwork
- idle or halt loop
- improved boot and initialization logging

**Status:** partially complete

**Completed so far:**

- explicit runtime initialization order in boot logs
- explicit exception and interrupt subsystem state
- first real breakpoint-handler path
- bounded QEMU validation for the breakpoint path

**Still deferred within this milestone:**

- broader hardware interrupt handling
- timer-driven runtime behavior beyond groundwork
- deeper interrupt controller work

**Acceptance criteria:**

- initialization order is visible in logs
- low-level runtime setup is easier to reason about
- failures are easier to diagnose

---

### Milestone 5 — Memory Management Foundation

**Goal:** introduce the smallest useful memory subsystem.

**Deliverables:**

- memory map handling
- frame allocator skeleton
- heap strategy decision
- minimal allocator only if justified

**Status:** complete enough for the current stage

**Completed so far:**

- real UEFI memory-map discovery
- host-testable discovered-memory summary
- minimal frame allocator seed derived from the first discovered conventional memory range
- explicit heap deferral

**Still deferred within this milestone:**

- real frame allocation behavior
- heap allocation
- allocator design work

**Acceptance criteria:**

- memory initialization is documented
- the design remains intentionally minimal
- no unnecessary generalization is introduced

---

### Milestone 6 — Unix-like Direction Definition

**Goal:** define the first Unix-like boundaries without pretending to be complete.

**Deliverables:**

- syscall direction note
- task or process model sketch
- VFS direction note
- user and kernel boundary plan

**Status:** in progress through the U-series phases

**Completed so far:**

- top-level Unix-like direction note
- syscall direction note
- task direction note
- descriptor direction note
- small code boundaries for syscall, task, and descriptor starters

**Still deferred within this milestone:**

- VFS direction note as a dedicated document
- real user-mode execution
- real syscall ABI entry
- process model beyond task-first direction

**Acceptance criteria:**

- the meaning of "Unix-like" is clearly defined for this project
- non-goals remain explicit
- the roadmap stays realistic and educational

## Unix-like MVP Implementation Phases

These U-series phases extend the earlier foundation milestones and provide the implementation order for the Unix-like MVP.

They should stay aligned with:

- `docs/unix-like.md`
- `docs/testing.md`
- `docs/blog-os-adoption.md`

### U1 — Testing Foundation and `blog_os` Adoption Plan

**Goal:** establish the testing and reference policy needed for deeper kernel work.

**Deliverables:**

- testing strategy
- `blog_os` adoption policy
- split host-side unit tests from bounded QEMU tests
- keep local and CI validation aligned around explicit workflow commands

**Status:** complete

**Acceptance criteria:**

- the testing strategy is documented clearly
- the `blog_os` adoption policy is documented clearly
- host-side unit tests and bounded QEMU tests are separate and explicit
- local and CI validation use the same workflow shape

---

### U2 — Exception and Interrupt Groundwork

**Goal:** introduce the smallest useful exception and interrupt foundation aligned with the current minimal kernel direction.

**Deliverables:**

- exception handling direction
- breakpoint and double-fault groundwork
- hardware interrupt groundwork
- timer-first interrupt direction
- host-testable interrupt state
- bounded QEMU validation for interrupt-related runtime behavior

**Status:** in progress

**Completed so far:**

- explicit exception and interrupt subsystem state
- host-side unit tests for pure interrupt logic
- boot logs that distinguish exception and interrupt initialization
- real breakpoint-handler groundwork

**Still deferred within this phase:**

- broader hardware interrupt handling
- double-fault completion work
- timer-driven runtime behavior beyond groundwork

**Acceptance criteria:**

- exception and interrupt groundwork are represented explicitly in code
- host-side unit tests cover the new pure logic
- boot logs show exception and interrupt initialization separately
- bounded QEMU validation still passes
- the implementation stays small and educational

---

### U3 — Controlled Exception Path

**Goal:** introduce the first narrow, testable exception path.

**Deliverables:**

- one controlled exception path
- bounded emulator validation for the chosen exception
- explicit success and failure reporting
- clearer direction for later double-fault handling

**Status:** complete for the first breakpoint path

**Acceptance criteria:**

- an explicit exception-test boot mode exists and is easy to trigger from the project workflow
- one real exception path is implemented and visible
- the exception path is validated through bounded QEMU testing
- the success marker is emitted by the real handler path, not by ordinary post-trigger control flow
- the implementation remains narrow and easy to reason about

---

### U4 — Real Memory Foundation

**Goal:** move from placeholder memory state to real discovered memory information.

**Deliverables:**

- real memory map boundary
- frame allocator groundwork backed by discovered memory
- host-side tests for memory bookkeeping where possible

**Status:** complete enough for the current stage

**Acceptance criteria:**

- memory work is backed by real discovered memory information
- the frame allocator direction is connected to actual memory information
- the implementation remains explicit and minimal

---

### U5 — Paging and Heap Direction

**Goal:** define the smallest useful paging and heap direction without introducing premature allocator or page-table complexity.

**Deliverables:**

- paging direction notes
- minimal host-testable paging helpers and state
- minimal kernel paging boundary
- small architecture-facing paging probe boundary
- explicit heap strategy decision
- allocator work only if justified by the code

**Status:** complete enough for the current stage

**Completed so far:**

- paging direction note
- host-testable paging helpers and state
- minimal kernel paging boundary
- small architecture-facing paging probe
- explicit heap deferral
- clear paging boot logs

**Still deferred within this phase:**

- page-table management
- mapping and unmapping APIs
- heap allocation
- allocator design work

**Acceptance criteria:**

- paging direction is documented clearly
- host-testable paging helpers and state are small, explicit, and covered by unit tests
- the kernel exposes a minimal paging subsystem boundary
- the architecture layer exposes only a small paging-facing probe, not full paging management
- paging groundwork is introduced only as needed
- heap support remains deferred unless justified by a concrete need

---

### U6 — Unix-like Kernel Boundary

**Goal:** make the Unix-like direction concrete through small kernel interfaces without introducing premature user-mode, scheduler, or VFS complexity.

**Deliverables:**

- syscall boundary notes
- minimal host-testable syscall number and result model
- minimal kernel syscall boundary
- task model sketch
- task direction notes
- minimal host-testable task state starter
- descriptor or handle direction
- descriptor direction notes
- minimal host-testable descriptor handle starter
- tiny task/descriptor ownership sketch
- VFS direction
- clearer user and kernel boundary planning

**Status:** in progress

**Completed so far:**

- syscall direction note
- task direction note
- descriptor direction note
- minimal host-testable syscall number, result, request, and dispatch model
- minimal kernel syscall boundary
- minimal host-testable task state starter
- minimal host-testable descriptor handle starter
- tiny task-to-descriptor ownership sketch
- kernel-facing syscall dispatch summary hook

**Still deferred within this phase:**

- real syscall ABI wiring
- user-mode execution
- scheduler and context switching
- descriptor tables
- VFS direction as a dedicated document
- richer ownership and lifecycle rules

**Acceptance criteria:**

- the Unix-like direction is reflected in small explicit kernel boundaries
- syscall, task, descriptor, and VFS direction are documented clearly
- host-testable syscall, task, and descriptor starters are small, explicit, and covered by unit tests
- the implementation order remains realistic and educational

## Suggested Release Shape

A practical early release sequence:

- `v0.1.0-alpha.1` — first public release of the repository foundation, docs, CI, and early bootable kernel workflow
- `v0.1.0` — documented reproducible QEMU boot path with polished onboarding and release assets
- `v0.2.0` — exception, interrupt, and memory foundation consolidated
- `v0.3.0` — Unix-like boundary notes and minimal interface starters consolidated

## Release Readiness Checklist

Before the first public release, the project should have:

- consolidated and consistent documentation
- clean contributor-facing onboarding
- passing CI
- passing bounded QEMU smoke validation
- release notes
- changelog
- community health files
- badges
- basic GitHub launch metadata

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
- the release strategy changes

## Related Documents

- `README.md`
- `docs/architecture.md`
- `docs/testing.md`
- `docs/unix-like.md`
- `docs/blog-os-adoption.md`
- `docs/paging.md`
- `docs/syscalls.md`
- `docs/tasks.md`
- `docs/descriptors.md`
- `docs/decisions/0001-target-platform.md`

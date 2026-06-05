# rustos Roadmap

Status legend:

- done — implemented and part of the current foundation
- current — the active area being strengthened now
- next — the most useful next step after the current milestone
- later — intentionally deferred

## Purpose

This document is the single source of truth for project status, milestone sequencing, and near-term priorities.

For the current implementation contract, see `docs/spec.md`.
For structural rules, see `docs/architecture.md`.

## Current snapshot

Current release target:

- `v0.1.0-alpha.1`

Current foundation status:

- the repository structure is in good shape
- the kernel boots in QEMU
- the exception smoke path uses a real breakpoint handler
- memory discovery and paging direction are in place
- the Unix-like foundation now includes syscall, task, descriptor, and VFS starter boundaries
- the docs are now centered on the core contract docs instead of many overlapping design notes

## Definition of done for a milestone

A milestone is done when:

- the code and docs agree
- the change is validated with the smallest useful checks
- boundaries are clearer, not blurrier
- new complexity is justified by a real need
- contributor entry points remain understandable

## Done milestones

### Milestone 0 — Repository and workflow foundation

Status: done.

Delivered:

- Rust workspace setup
- pinned toolchain
- CI workflow
- `xtask` workflow entry points
- contributor and project hygiene files

### Milestone 1 — Bootable kernel path

Status: done.

Delivered:

- `no_std` UEFI entrypoint
- early console output
- direct QEMU workflow
- deterministic boot path

### Milestone 2 — Exception and interrupt groundwork

Status: done enough for the current stage.

Delivered:

- explicit runtime initialization order
- host-testable interrupt and exception state
- real x86_64 breakpoint-handler path
- bounded breakpoint smoke testing

Still deferred inside this area:

- broader hardware interrupts
- complete double-fault strategy
- timer-driven runtime behavior

### Milestone 3 — Memory and paging foundation

Status: done enough for the current stage.

Delivered:

- real UEFI memory-map discovery
- first conventional-range capture
- frame-allocator seed derivation
- paging helpers and paging direction state
- x86_64 paging probe boundary
- explicit heap deferral

Still deferred inside this area:

- real frame allocation
- heap allocation
- page-table management

## Current milestone

### Milestone 4 — Unix-like boundary starter

Status: current.

Goal:

- make the Unix-like direction concrete through a few small code boundaries without pretending the kernel is already feature-complete

Delivered so far:

- syscall model and dispatch starter
- task identity and lifecycle starter
- descriptor handle and ownership starter
- VFS starter with:
  - path validation rules
  - root namespace entries
  - `/dev/console` device-path presence
- kernel-facing wrappers for syscall and VFS readiness
- documentation consolidated around the core project contract

Acceptance criteria for this milestone:

- syscall, task, descriptor, and VFS boundaries are explicit
- the code remains small and teachable
- the docs describe what exists now, not a future wish list
- no unnecessary allocator, scheduler, or filesystem machinery is introduced

## Next milestone candidates

### Next 1 — Broaden real runtime behavior only where it pays off

Priority: next.

Best candidates:

- make timer/interrupt groundwork slightly more concrete
- turn the frame-allocator seed into a small real allocator only if another subsystem needs it
- add one more bounded runtime smoke test only if it validates a real new boundary

### Next 2 — Real user/kernel execution boundary

Priority: next, but only after the current foundation stays stable.

Candidates:

- syscall entry/exit mechanics
- user/kernel memory boundary rules
- user-mode groundwork

This should happen only after the project has a concrete reason to own that complexity.

## Later milestones

### Later — Resource model growth

- descriptor tables
- real open/close behavior
- real VFS lookup and handle integration
- filesystems or device-backed I/O behavior

### Later — Execution model growth

- scheduler
- context switching
- multitasking
- address-space ownership beyond the current direction

### Later — Platform growth

- multi-architecture support
- broader hardware work outside the current emulator target

## Explicit non-goals for the current roadmap window

The project is still not trying to provide:

- production readiness
- full Unix compatibility
- broad platform support
- networking
- feature growth for its own sake

## Major decisions captured by this refactor

This refactor changes the roadmap in two important ways:

1. the documentation structure is now simpler and more authoritative
2. the VFS starter is now part of the implemented Unix-like boundary instead of remaining only a design direction item

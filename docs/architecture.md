# Architecture

Status: current working architecture after the 2026-06 foundation refactor.

This document describes the stable shape of the codebase: the layers that exist, the boundaries that matter, and the rules that should guide future changes.

For current implementation scope and subsystem behavior, see `docs/spec.md`.
For status and next steps, see `docs/roadmap.md`.

## Architectural goals

`rustos` is optimized for:

- small understandable modules
- explicit subsystem boundaries
- host-testable pure logic
- narrow unsafe and firmware-facing code
- reproducible local and CI workflows
- documentation that matches the codebase instead of getting ahead of it

## Workspace structure

### `kernel/`

`kernel/` contains runtime-facing code that depends on firmware, architecture, or the boot environment.

Current responsibilities:

- UEFI entry and boot-mode selection
- early console output
- architecture-facing interrupt and paging hooks
- runtime initialization sequencing
- kernel-facing wrappers for memory, paging, syscalls, and VFS
- panic marker and idle behavior

Rule: if a behavior requires UEFI services, CPU-specific setup, or runtime execution state, it belongs here.

### `nucleus/`

`nucleus/` contains host-testable pure logic.

It is now intentionally split into one source file per subsystem:

- `arch.rs`
- `console.rs`
- `interrupt.rs`
- `memory.rs`
- `paging.rs`
- `syscall.rs`
- `task.rs`
- `descriptor.rs`
- `vfs.rs`

This split is a deliberate refactor decision. The previous single oversized `lib.rs` file made the project harder to navigate and more difficult to evolve safely.

Rule: if logic can be validated as a normal Rust unit test on the host, it should prefer living in `nucleus/`.

### `xtask/`

`xtask/` is the supported workflow boundary.

It owns:

- formatting checks
- linting
- workspace checks
- host-side unit tests
- bounded QEMU smoke tests
- interactive local runs

Rule: repository workflows should stay explicit and Rust-native rather than spreading across ad hoc shell scripts.

### `docs/`

`docs/` is intentionally small and centered on a core set of documents:

- `README.md`
- `docs/spec.md`
- `docs/architecture.md`
- `docs/roadmap.md`
- `docs/decisions/`

This is also a deliberate refactor decision. The documentation structure now prefers one authoritative document per concern instead of many overlapping direction notes.

## Boundary model

### Boot boundary

The boot boundary is responsible for:

- entering through UEFI
- choosing the current boot mode
- running a visible initialization sequence
- keeping the boot path readable and diagnosable

The boot boundary should remain small and explicit.

### Architecture boundary

The architecture boundary isolates target-specific behavior.

Current responsibilities include:

- loading the x86_64 IDT
- triggering the real breakpoint path
- observing the current paging context through `CR3`
- exposing minimal runtime state to the rest of the kernel

The architecture boundary should not grow into a general abstraction layer before the project actually needs one.

### Runtime subsystem boundaries

The kernel currently exposes small subsystem boundaries for:

- `console`
- `interrupt`
- `memory`
- `paging`
- `panic`
- `syscall`
- `vfs`

These boundaries are intentionally narrow. They exist to make responsibilities clear, not to imply subsystem completeness.

### Host-testable logic boundary

The most important structural rule in the project is the split between:

- host-testable logic in `nucleus/`
- runtime-facing integration in `kernel/`

That split keeps:

- unit tests fast
- state modeling simple
- firmware-facing code smaller
- QEMU tests focused on real runtime behavior

## Current data and control flow

A normal boot currently follows this shape:

1. `main.rs` enters through UEFI and delegates to `kernel::boot`
2. `boot` initializes the console and selects a boot mode
3. `boot` runs the runtime sequence in a fixed visible order
4. runtime-facing modules call into `nucleus` for pure logic and summaries where appropriate
5. QEMU smoke tests validate the boot path or the controlled breakpoint path through `xtask`

## Placement rules for future code

When adding new code, prefer these rules:

1. put pure logic in `nucleus/`
2. keep runtime wrappers in `kernel/` small
3. isolate unsafe code near the boundary that requires it
4. add new modules only when they express a real boundary
5. do not introduce heap allocation until a concrete subsystem requires it
6. do not add generic infrastructure earlier than necessary

## Unix-like direction at the architectural level

`rustos` is moving toward a small Unix-like teaching kernel, but only through concrete boundaries.

That direction currently means:

- a minimal syscall model
- a minimal task model
- a minimal descriptor model
- a new VFS starter boundary

It does **not** currently mean:

- user mode
- a real scheduler
- a real descriptor table
- a real VFS implementation
- broad compatibility promises

## Documentation ownership

The documentation now follows this ownership model:

- `README.md` owns project overview and quick start
- `docs/spec.md` owns the current technical contract
- `docs/architecture.md` owns stable structure and engineering rules
- `docs/roadmap.md` owns status and sequencing
- `docs/decisions/` owns durable rationale

This keeps the project easier to understand and reduces stale duplication.

## Key decisions recorded here

This refactor makes three structural decisions explicit:

1. `nucleus` is a real module boundary, not a dumping ground for everything host-testable.
2. The first VFS boundary now exists in code as a minimal namespace contract.
3. The docs are intentionally consolidated around four core documents plus ADRs.

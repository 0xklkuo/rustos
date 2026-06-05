# rustos Specification

Status: current repository contract after the 2026-06 foundation refactor.

This document is the primary technical source of truth for what `rustos` is **today**. It is intentionally narrower than a design wish list and more concrete than a roadmap.

## Purpose

`rustos` is a minimal, educational operating system project for learning Rust systems programming and modern OS fundamentals through a real codebase.

The project favors:

- clarity over cleverness
- explicit boundaries over hidden behavior
- host-testable logic over emulator-only logic when possible
- focused milestones over speculative subsystem growth

## Product and platform contract

### Current target

`rustos` currently targets:

- `x86_64-unknown-uefi`

Current development assumptions:

- Apple Silicon macOS is a supported host workflow target
- QEMU is the primary execution and test environment
- the project uses nightly Rust because the real `x86-interrupt` breakpoint path still requires unstable ABI support

The rationale for this target is recorded in `docs/decisions/0001-target-platform.md`.

### Project scope

The current foundation is intentionally small. `rustos` aims to provide:

- a bootable kernel path
- explicit runtime subsystem boundaries
- a clean split between firmware-facing code and host-testable logic
- a minimal Unix-like direction without pretending to be complete
- a reproducible local and CI workflow

It does **not** currently aim to provide:

- a production operating system
- broad hardware support
- a stable user/kernel ABI
- user-mode execution
- multitasking
- filesystems or networking
- full VFS behavior
- POSIX compatibility claims

## Repository contract

The workspace is intentionally small:

- `kernel/` contains firmware-facing and runtime-facing code
- `nucleus/` contains host-testable pure logic and subsystem models
- `xtask/` contains the supported developer workflow entry points
- `docs/` contains the project contract, architecture, roadmap, and durable decisions

## Runtime contract

### Boot modes

The kernel currently supports two boot modes:

- `Normal`
- `ExceptionTest`

`ExceptionTest` is selected through a dedicated marker file so the exception smoke path stays narrow and explicit.

### Runtime initialization order

The normal boot path currently initializes and reports the following high-level sequence:

1. console
2. architecture groundwork
3. exception groundwork
4. interrupt groundwork
5. timer groundwork
6. memory discovery and frame-allocator seed derivation
7. paging direction and architecture probe
8. syscall boundary
9. VFS starter boundary
10. panic label and idle readiness

The runtime is still log-driven and intentionally simple. Most subsystems report readiness rather than performing deep runtime behavior.

## Subsystem contract

### Console

The console boundary currently provides:

- UEFI-backed early output
- explicit initialized versus deferred console state in `nucleus::console`
- plain-language boot log output through `kernel::console`

### Interrupt and exception groundwork

The interrupt boundary currently provides:

- host-testable interrupt and exception state in `nucleus::interrupt`
- a real x86_64 breakpoint handler path
- bounded QEMU validation for the controlled breakpoint path
- modeled timer interrupt readiness as groundwork, not as full timer-driven runtime behavior

What is still deferred:

- broader hardware interrupt handling
- complete double-fault strategy
- timer-driven scheduling or runtime behavior

### Memory

The memory boundary currently provides:

- real UEFI memory-map discovery in `kernel::memory`
- host-testable discovered-memory summaries in `nucleus::memory`
- capture of the first discovered conventional memory range
- a derived `FrameAllocatorSeed`
- a frame-allocator skeleton boundary
- explicit heap deferral

What is still deferred:

- real frame allocation
- heap allocation
- allocator policy work

### Paging

The paging boundary currently provides:

- host-testable address and page-range helpers in `nucleus::paging`
- explicit paging state (`Deferred`, `DirectionDefined`, `ArchProbeReady`)
- an architecture-facing x86_64 probe through `CR3`
- boot-time paging summaries in `kernel::paging`

What is still deferred:

- page-table ownership and management
- mapping and unmapping APIs
- page-fault handling as a paging subsystem milestone
- heap-backed paging metadata

### Syscall

The syscall boundary currently provides:

- host-testable syscall numbers in `nucleus::syscall`
  - `Write`
  - `Exit`
  - `Unknown(raw)`
- host-testable syscall errors
  - `InvalidNumber`
  - `InvalidArgument`
  - `InvalidHandle`
- a tiny request/dispatch model
- a kernel-side boundary marker in `kernel::syscall`

Current dispatch rules:

- `write` succeeds only when the handle is non-zero and the value is non-zero
- `exit` succeeds and returns the provided value
- unknown numbers fail with `InvalidNumber`

What is still deferred:

- a real trap/ABI entry path
- user-mode transitions
- pointer validation and copy-in/copy-out rules

### Task

The task boundary currently provides:

- a minimal `Id(usize)` where `0` is invalid
- task states:
  - `Created`
  - `Ready`
  - `Running`
  - `Exited`

This is a model boundary only. It does not imply real scheduling or context switching yet.

### Descriptor

The descriptor boundary currently provides:

- a minimal `Handle(u32)` where `0` is invalid
- a tiny ownership sketch connecting one valid task ID to one valid handle

It does **not** yet provide descriptor tables, lifecycle rules, or runtime-backed resources.

### VFS starter

The VFS boundary now provides a minimal starter model in `nucleus::vfs` and a kernel-facing readiness wrapper in `kernel::vfs`.

Current path contract:

- paths must be absolute
- empty paths are invalid
- empty path components are invalid

Current starter namespace:

- `/` → directory
- `/dev` → directory
- `/dev/console` → writable device node
- `/init` → file-like node

This is a namespace and validation boundary only. It is **not** a real VFS implementation.

What is still deferred:

- dynamic path resolution
- mount points
- open handle tables
- filesystem backends
- device model integration beyond the starter node shape

## Testing and workflow contract

The supported workflow entry points are the `xtask` commands:

- `check`
- `fmt`
- `lint`
- `test-unit`
- `test-qemu`
- `test-exception`
- `test`
- `run`

Testing is intentionally layered:

- pure logic should live in `nucleus/` and be tested on the host
- QEMU smoke tests should remain bounded and only cover behavior that truly depends on boot/runtime execution

Current QEMU success markers:

- normal boot: `rustos: hello from UEFI`
- controlled exception path: `rustos: breakpoint handler reached`

## Documentation contract

The documentation is intentionally centered on four core docs:

- `README.md` for orientation and quick start
- `docs/spec.md` for the current technical contract
- `docs/architecture.md` for structure and design rules
- `docs/roadmap.md` for status and sequencing

Decision records remain the right place for durable rationale that should outlive short-term implementation details.

## Decisions recorded in this refactor

This refactor makes the following decisions explicit:

1. `docs/spec.md` is now the main implementation contract for the current system.
2. `nucleus` is organized as one source file per subsystem instead of one oversized `lib.rs` file.
3. The Unix-like foundation now includes a small VFS starter in code, not just in design notes.
4. Heap allocation, real VFS behavior, real frame allocation, and real user-mode/syscall ABI work remain deferred until the codebase has a concrete need for them.

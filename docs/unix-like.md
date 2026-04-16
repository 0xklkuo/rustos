# Unix-like Direction

## Purpose

This document defines what "Unix-like" means for `rustos` during the current stage of the project.

The goal is not to claim Unix compatibility early. The goal is to define a small, realistic direction for future kernel work so contributors understand:

- what the project is aiming toward
- what is intentionally deferred
- which subsystem boundaries matter first
- how to avoid scope creep

`rustos` is still a minimal educational kernel. This document only sets direction. It does not imply that the features described here are already implemented.

## What "Unix-like" Means in `rustos`

For `rustos`, "Unix-like" means the kernel should gradually move toward a model with:

- a clear kernel and user boundary
- a syscall interface for controlled kernel services
- task or process execution units
- file-like interfaces for resources where that model is useful
- a small virtual filesystem direction
- simple, explicit ownership of kernel resources
- predictable error handling and blocking behavior

This does **not** mean:

- full POSIX compatibility
- a complete Unix userland
- shell support in the MVP
- signals, pipes, sockets, and process groups from day one
- compatibility with Linux, BSD, or macOS binaries
- copying mature Unix kernels feature-for-feature

The project should borrow useful Unix ideas without pretending to be a full Unix system early.

## Design Principles

### 1. Small interfaces first

Every new Unix-like boundary should start with the smallest useful interface.

Examples:
- a tiny syscall surface before a large syscall table
- a single task model before advanced scheduling
- a minimal VFS abstraction before multiple filesystem backends

### 2. Explicit kernel ownership

Kernel-managed resources should have clear ownership and lifetime rules.

This matters especially for:
- memory regions
- handles or descriptors
- task state
- filesystem nodes
- device access

### 3. Stable concepts before broad features

The project should define stable concepts before implementing many features.

Examples:
- define what a task is before adding multitasking
- define what a file handle is before adding many file operations
- define what a syscall result looks like before adding many syscalls

### 4. Educational clarity over compatibility claims

If a design is easier to teach and reason about, prefer that over early compatibility goals.

### 5. Defer complexity aggressively

If a subsystem requires many hidden rules to look "Unix-like", it is probably too early.

## Kernel and User Boundary

A Unix-like direction requires a clear distinction between:

- privileged kernel code
- unprivileged future user code

For `rustos`, this boundary should eventually define:

- how execution enters the kernel
- how user code requests services
- how memory ownership differs between kernel and user space
- how invalid requests are rejected safely

### Early Direction

Before real user mode exists, the project should still design with this future boundary in mind.

That means:
- avoid APIs that assume everything runs with kernel privilege forever
- keep syscall-like interfaces separate from internal kernel helpers
- avoid exposing raw internal kernel structures as public interfaces

## Syscall Direction

The syscall layer should be the first explicit Unix-like interface.

### Initial Goals

The early syscall design should answer:

- how syscall numbers are represented
- how arguments are passed
- how results and errors are returned
- how invalid syscalls are handled
- how the kernel keeps the interface small and understandable

### Recommended Early Shape

Start with a tiny syscall model such as:

- `write`
- `exit`
- one or two diagnostic or test syscalls if needed

This is enough to establish:
- calling convention direction
- error model
- user/kernel transition shape
- descriptor-like resource direction

### Error Model

Prefer a simple result model:
- success value
- small negative or tagged error space
- explicit invalid argument handling

Avoid large error frameworks early.

## Task and Process Model

The project should define a minimal execution model before implementing scheduling complexity.

### Recommended Early Concept

Start with a **task** abstraction, not a full process tree.

A task should eventually describe:
- execution context
- stack state
- address-space relationship
- scheduling state
- kernel-visible identifier

### Why Task-First

A task-first model is simpler because it avoids early commitment to:
- fork semantics
- process groups
- sessions
- signals
- parent-child lifecycle rules

Those can be introduced later if the project grows enough to justify them.

### Process Direction

A fuller process abstraction can come later once the project has:
- task switching groundwork
- memory space boundaries
- syscall entry and exit
- descriptor ownership rules

## Descriptor and Handle Direction

A Unix-like system benefits from a small resource handle model.

The project should eventually converge on a descriptor-like abstraction for:
- console I/O
- files
- devices
- pipes or streams later if justified

### Early Rule

Do not implement many descriptor types early.
Instead, define the smallest useful shared concept:
- an integer or typed handle
- ownership rules
- close or release behavior
- invalid handle behavior

## Virtual Filesystem Direction

The VFS should begin as a design boundary, not a large subsystem.

### Early Goals

The VFS direction should answer:
- what a node represents
- how paths are resolved
- how open handles relate to nodes
- how devices may appear in the namespace
- what operations are common across resource types

### Recommended Early Shape

Start with a tiny conceptual model:
- root namespace
- node type enum
- path lookup boundary
- open/read/write trait or operation set only if clearly needed

### What to Avoid Early

Avoid:
- mount namespaces
- permissions model complexity
- symlinks
- advanced caching
- multiple real filesystem implementations
- journaling
- full path normalization edge cases

## Console and I/O Direction

The current console output is firmware-backed and early-boot oriented.

A Unix-like direction suggests that later I/O should move toward:
- descriptor-based writes
- device-backed console abstraction
- separation between early boot logging and runtime I/O

The project should keep these two phases distinct:
1. early boot console
2. runtime I/O model

That separation will make later syscall and descriptor work cleaner.

## Memory Direction and Unix-like Relevance

Memory work is not "Unix-like" by itself, but it strongly affects the Unix-like direction.

The kernel will eventually need:
- kernel memory ownership rules
- user memory boundaries
- copy-in and copy-out rules for syscalls
- task address-space direction

Before that exists, the project should avoid pretending user-space semantics are already available.

## Scheduling Direction

Scheduling should remain intentionally simple at first.

### Recommended Early Direction

If scheduling is introduced, start with:
- cooperative or very simple preemptive groundwork
- one runnable queue model
- explicit task states

Avoid early support for:
- priorities
- fairness tuning
- CPU affinity
- load balancing
- real-time scheduling classes

## Security Direction

A Unix-like direction implies future privilege boundaries, but the MVP should stay modest.

Early security-related goals:
- reject invalid kernel requests safely
- keep unsafe boundaries documented
- avoid exposing raw internal state through public interfaces
- prepare for privilege separation without claiming it exists yet

Avoid early complexity such as:
- users and groups
- ACLs
- capabilities model
- sandboxing frameworks

## Recommended Milestone Order for Unix-like Growth

A practical order is:

1. syscall direction note
2. task abstraction sketch
3. descriptor or handle model
4. user/kernel boundary planning
5. VFS boundary sketch
6. simple runtime I/O path
7. user-mode execution groundwork

This order keeps the project understandable and avoids building subsystems in the wrong order.

## Explicit Non-Goals

The following are not goals for the current Unix-like direction stage:

- POSIX certification
- Linux compatibility
- ELF loader completeness
- shell implementation
- networking stack
- signals
- fork and exec semantics
- pipes and sockets
- permissions and users model
- multiple filesystems
- package management
- init system design

These may be explored later, but they should not shape the early kernel architecture prematurely.

## Decision Summary

For `rustos`, the Unix-like direction is:

- conceptual first
- interface-driven
- task-first
- syscall-first
- VFS-later
- compatibility-later
- educational always

The project should adopt Unix-like ideas as small, explicit kernel boundaries rather than as a promise of early feature completeness.

## Revisit Conditions

This document should be revisited when one or more of the following become true:

- the first syscall interface is implemented
- task switching becomes real kernel behavior
- user-mode execution is introduced
- a descriptor model is added
- a VFS boundary is added
- the project scope expands beyond the current MVP

## Related Documents

- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/decisions/0001-target-platform.md`

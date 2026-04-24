# Paging Direction

## Purpose

This document defines the paging direction for `rustos`.

The goal is to make the paging boundary explicit without pretending that full paging management already exists.

At this stage, `rustos` should:

- explain why paging matters
- define the smallest useful paging concepts
- expose a small code boundary for future paging work
- keep heap and allocator complexity deferred

This document is a design note, not a status report.

For implementation status and milestone progress, see `docs/roadmap.md`.

## Why Paging Matters

Paging is a core part of a modern kernel because it shapes how the system thinks about:

- virtual and physical memory
- kernel memory ownership
- future user and kernel isolation
- safe syscall memory boundaries
- task address-space direction
- controlled access to mapped regions

For `rustos`, paging matters because the project aims toward a small Unix-like kernel model with a clearer kernel and user boundary over time.

At the same time, paging is easy to overbuild too early. The project should introduce only the smallest useful paging direction before taking ownership of more complex memory-management behavior.

## Minimal Paging Model

The paging model should stay intentionally small.

A practical minimal state shape is:

- `Deferred`
- `DirectionDefined`
- `ArchProbeReady`

### Meaning of each state

- `Deferred`
  - paging work is not yet active
- `DirectionDefined`
  - the project has documented paging intent and small host-testable helpers
- `ArchProbeReady`
  - the project has a small architecture-facing runtime hook for paging-related observation

This keeps the boundary honest.

It avoids claiming that paging is fully initialized when the project has only defined the direction and a narrow architecture-facing probe.

## Address and Range Helpers

The paging boundary should prefer only small, host-testable helpers such as:

- `is_page_aligned`
- `align_down`
- `align_up`
- `page_count_for_bytes`
- `page_range`

These helpers are useful because they are:

- simple
- testable on the host
- directly relevant to future memory work
- easy for contributors to understand

They also fit the project rule of testing pure logic without requiring QEMU.

## Architecture-Facing Probe

The current `x86_64` paging hook should stay minimal.

Its role is to answer one narrow question:

> does the architecture layer have a real place to observe paging-related runtime state?

That is enough for this stage.

The probe may observe a small piece of runtime paging state, but it should not:

- modify mappings
- expose a large paging API
- introduce unsafe abstractions without need
- force a page-table design too early

This keeps the architecture boundary real without making the subsystem larger than the project currently needs.

## Relationship to UEFI

`rustos` currently boots through UEFI.

That means the kernel begins execution in an environment where paging is already meaningful at the platform level, but the project does not need to take over paging management immediately.

The important rule is:

- acknowledge the runtime paging context
- define the kernel-side boundary clearly
- defer direct paging control until the project has a concrete reason to own it

This is more honest and more educational than pretending the kernel must immediately implement a full paging subsystem.

## Heap Strategy

The heap strategy for the current paging direction is simple:

- heap support remains deferred

This is an intentional design choice.

Paging work often leads projects to introduce allocation too early. `rustos` should avoid that.

### Why heap remains deferred

Right now, the project does not yet need dynamic allocation for:

- page-table management
- task tables
- descriptor tables
- VFS structures
- runtime object graphs

Adding heap support now would create complexity without solving a real problem.

### Current rule

Do not add heap or allocator work just because paging exists as a concept.

Only revisit heap support when a concrete subsystem requires it.

Examples of valid future triggers:

- real paging metadata cannot stay static
- descriptor tables need dynamic growth
- task management needs runtime-owned collections
- VFS work needs dynamic node or handle storage

Until then, the correct strategy is still:

- `Deferred`

## What Is Explicitly Deferred

The following are intentionally deferred for now:

- page-table management
- mapping and unmapping APIs
- higher-half kernel design decisions
- user-space address-space creation
- copy-in and copy-out helpers
- page-fault handling as a paging milestone
- heap-backed paging metadata
- dynamic allocation for paging structures
- allocator design work
- global allocator setup
- `alloc`-based kernel collections

These are all valid future topics, but they should not be introduced before the project has a concrete need and a clear teaching value.

## Relationships to Other Boundaries

Paging affects several other subsystem directions.

### Memory

Paging builds on the memory foundation, but it should not replace it.

The memory boundary should remain responsible for:

- discovered memory information
- frame allocator direction
- heap deferral

The paging boundary should remain responsible for:

- virtual-memory direction
- page-alignment helpers
- architecture-facing paging observation
- future mapping direction

### Syscalls

Paging will eventually matter for:

- user and kernel memory separation
- pointer validation
- copy-in and copy-out rules

Those concerns should remain deferred until the syscall boundary becomes more concrete.

### Tasks

Paging will eventually matter for:

- task address-space relationships
- user-mode execution
- memory ownership boundaries

Those concerns should remain deferred until the task model grows beyond its current minimal state.

## Testing Direction

Paging work should follow the existing project testing strategy.

### Test on the host first

Prefer unit tests for:

- alignment helpers
- page-count helpers
- page-range helpers
- paging state summaries
- deferred heap strategy summaries

### Use emulator tests only when needed

Do not add QEMU paging tests unless the kernel begins to change real runtime paging behavior in a meaningful and observable way.

For the current paging direction, host-side tests are the right default.

## Design Rules for Contributors

When extending paging work in `rustos`, follow these rules:

1. keep the paging boundary small
2. prefer host-testable pure logic first
3. do not introduce heap allocation without a concrete need
4. do not add mapping APIs before the project needs them
5. keep architecture-specific code behind the architecture boundary
6. document unsafe invariants clearly if low-level paging code is added later
7. avoid copying a larger reference design mechanically
8. make design claims match the real implementation

## What Future Paging Work Might Add

Later work may introduce some of the following, if justified:

- explicit page-table ownership rules
- kernel mapping policy
- page-fault handling tied to real paging behavior
- user and kernel address-space separation
- copy-in and copy-out helpers for syscalls
- a minimal mapper interface
- frame allocation tied to real mapping work

These should be added only when the surrounding kernel design is ready.

## Decision Summary

For `rustos`, paging is:

- documented clearly
- represented by a small code boundary
- supported by host-testable helpers
- connected to a minimal architecture-facing probe
- not yet a full paging implementation

The heap strategy is:

- explicitly deferred

This keeps the project aligned with its core principles:

- minimal first
- clarity over cleverness
- explicit boundaries
- no premature complexity
- educational value over feature count

## Related Documents

- `docs/architecture.md`
- `docs/roadmap.md`
- `docs/testing.md`
- `docs/unix-like.md`
- `docs/syscalls.md`
- `docs/tasks.md`
- `docs/descriptors.md`
- `docs/blog-os-adoption.md`

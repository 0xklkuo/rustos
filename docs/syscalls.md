# Syscall Direction

## Purpose

This document defines the syscall direction for `rustos` during the current U6 milestone.

The goal is to make the first Unix-like kernel boundary explicit without pretending that real user-mode execution or a full syscall ABI already exists.

At this stage, `rustos` should:

- define the smallest useful syscall model
- keep the syscall boundary separate from internal kernel helpers
- make the error model explicit
- keep the implementation small and teachable
- avoid introducing scheduler, process, or VFS complexity too early

This document is direction-first. It does not claim that `rustos` already supports real user-space syscalls.

## Why Syscalls Matter

A Unix-like kernel needs a controlled boundary between:

- unprivileged requests
- privileged kernel behavior

That boundary is the syscall layer.

For `rustos`, syscalls matter because they shape how the project will eventually define:

- kernel and user responsibilities
- service entry into the kernel
- argument validation
- error reporting
- descriptor-based resource access
- task lifecycle requests such as exit

A small syscall model is enough to begin defining these ideas clearly.

## Current Milestone Goal

The current U6 goal is:

- define syscall direction clearly in documentation
- add a host-testable syscall number model
- add a host-testable syscall result and error model
- add a tiny host-testable syscall dispatch sketch
- add a small kernel syscall boundary module
- keep task and descriptor direction minimal and explicit
- defer real syscall ABI wiring and user-mode execution

This means the milestone is about interface clarity, not full runtime behavior.

## What Exists Now

For the current milestone, `rustos` may expose:

- a small syscall number enum
- a small syscall error enum
- a small syscall result type
- a small syscall request type
- a decode helper for raw syscall numbers
- a tiny dispatch helper for host-side validation
- a kernel syscall boundary module
- plain-language summaries for early validation and teaching

These pieces are intentionally small.

They exist to define the first Unix-like interface shape, not to claim that the kernel already handles real syscall traps or transitions from user mode.

## What Is Explicitly Deferred

The following are intentionally deferred for now:

- real syscall entry instructions
- architecture-specific syscall ABI wiring
- user-mode execution
- register-based argument passing implementation
- syscall dispatch tables with many entries
- copy-in and copy-out helpers
- pointer validation against user memory
- process model complexity
- scheduler integration
- signal handling
- permissions and credential checks
- VFS-backed syscall behavior
- compatibility with Linux, BSD, or POSIX ABIs

These are all valid future topics, but they should not be introduced before the project has a concrete need and a clear teaching value.

## Minimal Syscall Model

The current syscall model should stay intentionally small.

A practical minimal syscall set is:

- `write`
- `exit`

This is enough to define:

- syscall number representation
- success and failure results
- invalid syscall handling
- a first dispatch shape
- descriptor-like resource direction
- task lifecycle direction

### Why Start This Small

A tiny syscall set is useful because it:

- keeps the interface easy to understand
- avoids premature dispatch complexity
- supports future console and descriptor work
- supports future task lifecycle work
- gives contributors a concrete Unix-like boundary to study

## Syscall Number Direction

The syscall number model should answer one narrow question first:

> how does the kernel identify which service was requested?

For the current milestone, a small enum is enough.

A practical early shape is:

- `Write`
- `Exit`
- `Unknown(raw)`

This keeps the model explicit and avoids pretending that a large syscall table already exists.

### Decode Rule

The current decode rule should be simple:

- known raw values map to known syscall numbers
- unknown raw values remain explicit as `Unknown(raw)`

This makes invalid syscall handling easy to test and easy to explain.

## Error Model

The syscall error model should stay intentionally small.

A practical early error set is:

- `InvalidNumber`
- `InvalidArgument`
- `InvalidHandle`

This is enough to define the first important failure cases:

- the syscall does not exist
- the syscall arguments are not valid
- the descriptor-like handle is not valid

### Why Keep Errors Small

A small error model is better at this stage because it:

- keeps tests simple
- keeps summaries readable
- avoids large error frameworks
- matches the current educational scope

## Result Model

The syscall result model should answer:

- was the syscall successful?
- if successful, what value was returned?
- if not successful, what error occurred?

A small result type with:

- a success value
- an optional error

is enough for the current milestone.

This keeps the model explicit without forcing a larger ABI encoding decision yet.

## Tiny Dispatch Sketch

The current U6.1 refinement adds a tiny host-testable dispatch sketch.

Its purpose is not to model a real syscall ABI.
Its purpose is to make the syscall boundary slightly more concrete while keeping all logic easy to test on the host.

A practical minimal request shape is:

- syscall number
- descriptor-like handle
- one small value field

For the current milestone, that value field can stand in for:

- byte count for `write`
- exit code for `exit`

This keeps the dispatch model intentionally small and avoids pretending that register-based argument passing already exists.

### Current Dispatch Rules

The current dispatch sketch should stay narrow and explicit:

- `write`
  - fail with `InvalidHandle` when the handle is zero
  - fail with `InvalidArgument` when the byte count is zero
  - otherwise succeed and return the byte count
- `exit`
  - succeed and return the provided exit code as the success value
- `Unknown(raw)`
  - fail with `InvalidNumber`

These rules are intentionally simple.

They are useful because they:
- make the first syscall behavior boundary concrete
- keep validation logic easy to understand
- avoid introducing dispatch-table or ABI complexity too early
- give contributors a small pure-logic model to test and extend later

## Kernel Boundary Rule

The syscall boundary should remain separate from internal kernel helpers.

That means:

- syscall-facing types should be small and explicit
- internal kernel helpers should not be treated as public syscall interfaces
- the kernel boundary should describe intent before it grows into a larger dispatch path

This is important even before real user mode exists.

It helps prevent the project from accidentally designing everything as if all callers are privileged forever.

## Relationship to Tasks

The syscall layer and task model are closely related, but they should not be merged too early.

For the current milestone:

- `exit` is enough to point toward task lifecycle direction
- task scheduling and switching remain deferred
- the task model should stay small and host-testable

This keeps the syscall boundary focused.

## Relationship to Descriptors

The syscall layer and descriptor model are also closely related.

For the current milestone:

- `write` is enough to point toward descriptor-like resource access
- descriptor ownership and release rules can stay minimal
- many descriptor types should remain deferred

This keeps the resource model small and understandable.

## Relationship to VFS

The syscall layer will eventually interact with the VFS, but not yet.

For the current milestone:

- the syscall model should not assume a full filesystem exists
- `write` should be treated as an interface direction, not as proof of file I/O support
- VFS work should remain a separate later boundary

This avoids pulling too much design into U6.

## Testing Direction

Syscall work should follow the existing project testing strategy.

### Test on the Host First

Prefer unit tests for:

- syscall number decoding
- known and unknown syscall handling
- syscall result success and failure behavior
- syscall request field access
- tiny dispatch success and failure behavior
- syscall summary helpers
- invalid argument and invalid handle cases

### Use Emulator Tests Only When Needed

Do not add QEMU syscall tests until the kernel has real runtime syscall behavior that can be observed meaningfully.

For the current milestone, host-side tests are the right default.

## Design Rules for Contributors

When extending syscall work in `rustos`, follow these rules:

1. keep the syscall surface small
2. prefer host-testable pure logic first
3. keep syscall-facing types separate from internal helpers
4. do not add many syscall numbers before the first ones are justified
5. keep the error model small and explicit
6. avoid ABI-specific complexity until the architecture boundary needs it
7. do not claim user-mode support before it exists
8. make milestone claims match the real implementation

## What Future Syscall Work Might Add

Later milestones may introduce some of the following, if justified:

- architecture-specific syscall entry and exit
- register-based argument passing rules
- richer syscall dispatch logic
- pointer validation rules
- copy-in and copy-out helpers
- descriptor-backed console writes
- task exit handling tied to real task state
- user and kernel memory boundary checks
- VFS-backed open, read, and write operations

These should be added only when the surrounding kernel design is ready.

## Decision Summary

For the current `rustos` milestone, the syscall direction is:

- documented clearly
- represented by a small code boundary
- supported by host-testable pure logic
- intentionally separate from internal kernel helpers
- not yet a real syscall ABI implementation

This keeps the project aligned with its core principles:

- minimal first
- clarity over cleverness
- explicit boundaries
- no premature complexity
- educational value over feature count

## Related Documents

- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/unix-like.md`
- `docs/testing.md`
- `docs/paging.md`

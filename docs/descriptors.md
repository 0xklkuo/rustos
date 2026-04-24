# Descriptor Direction

## Purpose

This document defines the descriptor direction for `rustos` during the current U6 milestone.

The goal is to make the descriptor-like resource boundary explicit without pretending that a full file-descriptor table, device model, or VFS-backed runtime already exists.

At this stage, `rustos` should:

- define the smallest useful handle model
- keep descriptor-facing concepts separate from internal kernel helpers
- make validity and ownership direction explicit
- keep the implementation small and teachable
- avoid introducing file, device, or VFS complexity too early

This document is direction-first. It does not claim that `rustos` already supports real descriptor tables or Unix-compatible file descriptor behavior.

## Why Descriptors Matter

A Unix-like kernel benefits from a small, consistent way to refer to kernel-managed resources.

That resource boundary often appears as a descriptor or handle model.

For `rustos`, descriptors matter because they shape how the project will eventually define:

- access to console and runtime I/O
- access to files and devices later
- resource ownership and release rules
- syscall-facing resource references
- invalid resource handling
- separation between public kernel interfaces and internal structures

A small descriptor model is enough to begin defining these ideas clearly.

## Current Milestone Goal

The current U6 goal is:

- define descriptor direction clearly in documentation
- add a host-testable descriptor-like handle starter
- add a tiny host-testable task-to-descriptor ownership sketch
- keep the descriptor boundary separate from internal kernel helpers
- keep ownership and release direction explicit
- defer real descriptor tables, VFS integration, and device models

This means the milestone is about interface clarity, not full runtime behavior.

## What Exists Now

For the current milestone, `rustos` may expose:

- a small descriptor-like handle type
- a simple validity rule
- a tiny task-to-descriptor ownership sketch
- a plain-language summary helper
- host-side unit tests for the minimal handle and ownership behavior

These pieces are intentionally small.

They exist to define the first resource-handle boundary, not to claim that the kernel already manages real open files, devices, or streams.

## What Is Explicitly Deferred

The following are intentionally deferred for now:

- real descriptor tables
- richer per-task descriptor ownership models
- open and close syscalls
- read and write implementations backed by real resources
- file-backed descriptors
- device-backed descriptors
- pipes, sockets, and streams
- descriptor duplication rules
- descriptor inheritance rules
- blocking and non-blocking behavior
- permissions and access control
- VFS-backed descriptor resolution
- Unix compatibility claims

These are all valid future topics, but they should not be introduced before the project has a concrete need and a clear teaching value.

## Minimal Descriptor Model

The current descriptor model should stay intentionally small.

A practical minimal shape is:

- one small handle type
- one simple validity rule
- one small summary helper

For example, the current direction can treat a descriptor-like handle as:

- a small integer value
- invalid when zero
- valid when non-zero

This is enough to define the first important boundary:

- kernel-facing resource references should be explicit
- invalid handles should be easy to detect
- future syscall work can point toward descriptor-based resource access

## Why Start This Small

A tiny descriptor model is useful because it:

- keeps the interface easy to understand
- avoids premature table-management complexity
- supports future syscall work such as `write`
- supports future console and device direction
- gives contributors a concrete Unix-like resource boundary to study

## Handle Direction

The handle model should answer one narrow question first:

> how does the kernel refer to a resource through a small public-facing identifier?

For the current milestone, a small typed wrapper is enough.

A practical early shape is:

- `Handle(u32)`

This keeps the model explicit and avoids exposing raw internal kernel structures directly.

### Validity Rule

The current validity rule should be simple:

- `0` is invalid
- non-zero values are valid

This rule is intentionally small.

It is not a final descriptor policy.
It is only the smallest useful rule for the current milestone.

## Ownership Direction

The descriptor boundary should eventually support clear ownership rules.

For the current milestone, the important direction is:

- a handle is not the resource itself
- a handle refers to a kernel-managed resource boundary
- internal kernel resource state should remain separate from the public-facing handle value

This distinction matters even before real descriptor tables exist.

It helps prevent the project from accidentally treating raw internal state as if it were already a stable public interface.

### Tiny Ownership Sketch

The current U6.3 refinement adds a tiny host-testable ownership sketch.

Its purpose is not to model a real descriptor table.
Its purpose is to make the relationship between tasks and descriptor-like handles slightly more concrete while keeping the model easy to test on the host.

A practical minimal ownership shape is:

- one task identifier
- one descriptor-like handle

This keeps the ownership model intentionally small and avoids pretending that the kernel already has:

- descriptor tables
- per-task descriptor maps
- inheritance rules
- release rules
- allocation policies

### Current Ownership Rule

The current ownership sketch should stay narrow and explicit:

- ownership is valid only when the task identifier is valid
- ownership is valid only when the descriptor-like handle is valid
- otherwise ownership is invalid

These rules are intentionally simple.

They are useful because they:
- make the first task-to-descriptor relationship concrete
- keep ownership validation easy to understand
- avoid introducing table-management complexity too early
- give contributors a small pure-logic model to test and extend later

## Release Direction

A Unix-like descriptor model eventually needs release behavior.

For the current milestone, release behavior should remain conceptual only.

The important early rule is:

- future descriptor work should define explicit close or release behavior
- invalid-handle behavior should be explicit
- ownership and release should be documented before many descriptor types are added

This keeps the model honest and avoids pretending that resource lifecycle rules already exist.

## Relationship to Syscalls

The descriptor model and syscall layer are closely related.

For the current milestone:

- `write` is enough to point toward descriptor-based resource access
- invalid handle behavior is enough to define one important syscall failure case
- real descriptor-backed I/O remains deferred

This keeps the descriptor direction small while still making the Unix-like boundary more concrete.

## Relationship to Tasks

The descriptor model and task model are also related, but they should not be merged too early.

For the current milestone:

- the tiny ownership sketch should not yet imply per-task descriptor tables
- task lifecycle should not yet imply descriptor inheritance or cleanup rules
- those rules should be introduced only when task and syscall behavior become more concrete

This keeps the current milestone focused.

## Relationship to VFS

The descriptor model will eventually interact with the VFS, but not yet.

For the current milestone:

- the descriptor model should not assume a full filesystem exists
- handles should be treated as a resource boundary direction, not as proof of file support
- VFS work should remain a separate later boundary

This avoids pulling too much design into U6.

## Testing Direction

Descriptor work should follow the existing project testing strategy.

### Test on the Host First

Prefer unit tests for:

- handle creation
- raw handle access
- validity checks
- invalid handle behavior
- ownership validity checks
- ownership summary helpers
- summary helpers

### Use Emulator Tests Only When Needed

Do not add QEMU descriptor tests until the kernel has real runtime descriptor behavior that can be observed meaningfully.

For the current milestone, host-side tests are the right default.

## Design Rules for Contributors

When extending descriptor work in `rustos`, follow these rules:

1. keep the descriptor model small
2. prefer host-testable pure logic first
3. keep descriptor-facing types separate from internal kernel helpers
4. do not add many descriptor kinds before the first ones are justified
5. keep invalid-handle behavior explicit
6. do not expose raw internal kernel resource structures as public interfaces
7. avoid VFS or device complexity until the surrounding kernel design needs it
8. make milestone claims match the real implementation

## What Future Descriptor Work Might Add

Later milestones may introduce some of the following, if justified:

- richer per-task descriptor ownership models
- per-task descriptor tables
- explicit close or release behavior
- descriptor allocation rules
- descriptor reuse rules
- console-backed descriptors
- file-backed descriptors
- device-backed descriptors
- descriptor duplication
- descriptor inheritance rules
- blocking and non-blocking behavior
- VFS-backed open, read, write, and close operations

These should be added only when the surrounding kernel design is ready.

## Decision Summary

For the current `rustos` milestone, the descriptor direction is:

- documented clearly
- represented by a small host-testable handle type
- extended by a tiny host-testable ownership sketch
- intentionally separate from internal kernel helpers
- useful for future syscall and resource-boundary work
- not yet a real descriptor-table implementation

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
- `docs/syscalls.md`
- `docs/paging.md`

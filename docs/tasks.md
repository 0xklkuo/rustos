# Task Direction

## Purpose

This document defines the task direction for `rustos`.

The goal is to make the task boundary explicit without pretending that real scheduling, context switching, or process management already exists.

## Why Tasks Matter

A Unix-like kernel needs a clear execution model.

For `rustos`, a task is the smallest useful unit for describing:

- kernel-visible execution identity
- lifecycle state
- future scheduling direction
- future syscall ownership
- future descriptor ownership
- future address-space relationships

A small task model is enough to begin defining these ideas clearly.

## Minimal Task Model

The task model should stay intentionally small.

A practical minimal shape is:

- a kernel-visible identifier
- a small lifecycle state

This is enough to define:

- whether a task identity is valid
- whether a task has only been created
- whether a task is ready to run
- whether a task is currently running
- whether a task has exited

### Task Identifier

The task identifier model should answer one narrow question first:

> how does the kernel refer to a task in a small, explicit way?

For the current direction, a small integer-backed identifier is enough.

A practical early shape is:

- `Id(usize)`

#### Validity Rule

The validity rule should stay simple:

- zero is invalid
- non-zero values are valid

This keeps the model easy to test and easy to explain.

It also avoids introducing allocation or global task tables too early.

### Task State

The task state model should stay intentionally small.

A practical early state set is:

- `Created`
- `Ready`
- `Running`
- `Exited`

#### Meaning of Each State

- `Created`
  - the task exists, but is not yet ready to run
- `Ready`
  - the task is eligible to run
- `Running`
  - the task is currently executing
- `Exited`
  - the task has completed execution

This is enough to define the first lifecycle boundary without forcing a scheduler design.

### Lifecycle Direction

The current direction does not need a full transition engine, but it should still imply a simple lifecycle shape.

A practical early direction is:

- `Created -> Ready`
- `Ready -> Running`
- `Running -> Exited`

This is only a conceptual direction for now.

It should not be treated as proof that the kernel already performs these transitions at runtime.

## Relationship to Other Boundaries

### Syscalls

The task model and syscall model are closely related.

For the current direction:

- `exit` is enough to point toward task lifecycle direction
- syscall ownership can remain conceptual
- task state should remain host-testable and independent from ABI details

This keeps the task model focused and avoids mixing it with trap-entry complexity.

### Descriptors

Tasks will eventually own or reference descriptor-like resources.

For the current direction:

- descriptor ownership should remain minimal and explicit
- task identity should not yet imply a descriptor table
- one tiny ownership sketch may connect a valid task identifier to a valid descriptor-like handle
- task and descriptor models should stay separate and small beyond that narrow boundary

This avoids introducing resource-management complexity too early.

### Memory

Tasks will eventually relate to memory ownership and address spaces.

For the current direction:

- task identity should not imply a private address space
- task state should not imply memory switching
- address-space relationships should remain deferred

This keeps the task model honest and avoids pretending that user-mode execution already exists.

### Scheduling

Scheduling should remain a separate later boundary.

For the current direction:

- task state exists
- scheduler policy does not
- runnable queues do not
- preemption does not
- fairness rules do not

This separation is important.

It prevents the project from overbuilding the task model before the kernel is ready for real runtime scheduling behavior.

## What Is Deferred

The following are intentionally deferred:

- scheduler implementation
- context switching
- timer-driven preemption
- runnable queues
- process trees
- parent-child lifecycle rules
- fork semantics
- exec semantics
- signals
- waiting and reaping
- task priorities
- CPU affinity
- load balancing
- user-mode task execution
- task address-space switching

These are all valid future topics, but they should not be introduced before the project has a concrete need and a clear teaching value.

## Design Rules

When extending task work in `rustos`, follow these rules:

1. keep the task model small
2. prefer host-testable pure logic first
3. keep task state separate from scheduler policy
4. do not introduce process-tree complexity early
5. do not imply user-mode execution before it exists
6. avoid hidden lifecycle rules
7. make milestone claims match the real implementation
8. add runtime complexity only when the surrounding kernel design is ready

## What Future Task Work Might Add

Later work may introduce some of the following, if justified:

- explicit task transition helpers
- runnable queue direction
- scheduler policy
- timer-driven preemption groundwork
- richer task ownership of descriptors
- task ownership of address spaces
- task exit handling tied to real kernel state
- waiting and reaping rules
- process abstraction layered on top of tasks
- user-mode task execution

These should be added only when the surrounding kernel design is ready.

## Decision Summary

For `rustos`, the task direction is:

- documented clearly
- represented by a small host-testable model
- connected to a tiny ownership sketch for descriptor-like handles
- intentionally separate from scheduling
- intentionally separate from process complexity
- not yet a real multitasking implementation

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
- `docs/syscalls.md`
- `docs/descriptors.md`
- `docs/testing.md`

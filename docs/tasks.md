# Task Direction

## Purpose

This document defines the task direction for `rustos` during the current U6 milestone.

The goal is to make the task boundary explicit without pretending that real scheduling, context switching, or process management already exists.

At this stage, `rustos` should:

- define the smallest useful task model
- keep task state explicit and easy to test
- separate task direction from scheduler complexity
- keep the implementation small and teachable
- avoid introducing process-tree, signal, or lifecycle complexity too early

This document is direction-first. It does not claim that `rustos` already supports multitasking.

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

## Current Milestone Goal

The current U6 goal is:

- define task direction clearly in documentation
- add a host-testable task identifier
- add a host-testable task state model
- keep task behavior separate from scheduling
- defer context switching and process management

This means the milestone is about interface clarity, not runtime multitasking.

## What Exists Now

For the current milestone, `rustos` may expose:

- a small task identifier type
- a small task state enum
- plain-language summaries for task state
- host-side unit tests for task identity and state

These pieces are intentionally small.

They exist to define the first task boundary, not to claim that the kernel already schedules or switches between tasks.

## What Is Explicitly Deferred

The following are intentionally deferred for now:

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

## Minimal Task Model

The current task model should stay intentionally small.

A practical minimal task shape is:

- a kernel-visible identifier
- a small lifecycle state

This is enough to define:

- whether a task identity is valid
- whether a task has only been created
- whether a task is ready to run
- whether a task is currently running
- whether a task has exited

### Why Start This Small

A tiny task model is useful because it:

- keeps the execution model easy to understand
- avoids premature scheduler design
- supports future syscall ownership direction
- supports future descriptor ownership direction
- gives contributors a concrete Unix-like boundary to study

## Task Identifier Direction

The task identifier model should answer one narrow question first:

> how does the kernel refer to a task in a small, explicit way?

For the current milestone, a small integer-backed identifier is enough.

A practical early shape is:

- `Id(usize)`

### Validity Rule

The current validity rule should be simple:

- zero is invalid
- non-zero values are valid

This keeps the model easy to test and easy to explain.

It also avoids introducing allocation or global task tables too early.

## Task State Direction

The task state model should stay intentionally small.

A practical early state set is:

- `Created`
- `Ready`
- `Running`
- `Exited`

### Meaning of Each State

- `Created`
  - the task exists, but is not yet ready to run
- `Ready`
  - the task is eligible to run
- `Running`
  - the task is currently executing
- `Exited`
  - the task has completed execution

This is enough to define the first lifecycle boundary without forcing a scheduler design.

## State Transition Direction

The current milestone does not need a full transition engine, but it should still imply a simple lifecycle direction.

A practical early direction is:

- `Created -> Ready`
- `Ready -> Running`
- `Running -> Exited`

This is only a conceptual direction for now.

It should not be treated as proof that the kernel already performs these transitions at runtime.

## Relationship to Syscalls

The task model and syscall model are closely related.

For the current milestone:

- `exit` is enough to point toward task lifecycle direction
- syscall ownership can remain conceptual
- task state should remain host-testable and independent from ABI details

This keeps the task model focused and avoids mixing it with trap-entry complexity.

## Relationship to Descriptors

Tasks will eventually own or reference descriptor-like resources.

For the current milestone:

- descriptor ownership should remain conceptual
- task identity should not yet imply a descriptor table
- task and descriptor models should stay separate and small

This avoids introducing resource-management complexity too early.

## Relationship to Memory

Tasks will eventually relate to memory ownership and address spaces.

For the current milestone:

- task identity should not imply a private address space
- task state should not imply memory switching
- address-space relationships should remain deferred

This keeps the task model honest and avoids pretending that user-mode execution already exists.

## Relationship to Scheduling

Scheduling should remain a separate later boundary.

For the current milestone:

- task state exists
- scheduler policy does not
- runnable queues do not
- preemption does not
- fairness rules do not

This separation is important.

It prevents the project from overbuilding the task model before the kernel is ready for real runtime scheduling behavior.

## Testing Direction

Task work should follow the existing project testing strategy.

### Test on the Host First

Prefer unit tests for:

- task identifier validity
- raw task identifier access
- task state summaries
- simple lifecycle meaning

### Use Emulator Tests Only When Needed

Do not add QEMU task tests until the kernel has real runtime task behavior that can be observed meaningfully.

For the current milestone, host-side tests are the right default.

## Design Rules for Contributors

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

Later milestones may introduce some of the following, if justified:

- explicit task transition helpers
- runnable queue direction
- scheduler policy
- timer-driven preemption groundwork
- task ownership of descriptors
- task ownership of address spaces
- task exit handling tied to real kernel state
- waiting and reaping rules
- process abstraction layered on top of tasks
- user-mode task execution

These should be added only when the surrounding kernel design is ready.

## Decision Summary

For the current `rustos` milestone, the task direction is:

- documented clearly
- represented by a small host-testable model
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
- `docs/testing.md`

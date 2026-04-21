# `blog_os` Adoption Policy

## Purpose

This document defines how `rustos` uses [`blog_os`](https://github.com/phil-opp/blog_os) as a reference.

`blog_os` is one of the best educational Rust OS resources available. It is highly valuable for learning low-level concepts, implementation patterns, and testing approaches. However, `rustos` does not aim to reproduce `blog_os` chapter-by-chapter or mirror its architecture exactly.

Instead, `rustos` uses `blog_os` selectively.

The goal of this policy is to keep that selection explicit so contributors understand:

- which `blog_os` ideas we want to adopt
- which ideas we want to adapt
- which ideas we want to defer
- which ideas do not fit the current project direction

## Core Rule

Use `blog_os` as a:

- concept reference
- low-level implementation reference
- testing inspiration
- debugging aid

Do not use `blog_os` as a:

- strict architecture template
- requirement to match chapter order
- reason to add complexity that does not fit `rustos`
- promise of feature parity

## Selection Criteria

A `blog_os` concept is a good fit for `rustos` when it:

- supports the current milestone directly
- improves educational clarity
- keeps the codebase small and understandable
- fits the current UEFI-first workflow
- can be tested clearly
- does not force premature subsystem complexity

A `blog_os` concept is a poor fit for the current stage when it:

- assumes a different boot model too strongly
- adds large abstractions before they are needed
- increases complexity without improving learning value
- pushes the project toward feature completeness too early
- conflicts with the current minimal Unix-like MVP direction

## Adoption Categories

Each referenced `blog_os` topic should fall into one of these categories:

### Adopt

Use the concept mostly as-is, with only small project-specific adjustments.

### Adapt

Use the concept, but reshape it to fit `rustos` goals, current architecture, or workflow.

### Defer

The concept is useful, but not yet appropriate for the current milestone.

### Reject for now

The concept is not a good fit for the current project stage or direction.

This is not a permanent rejection. It only means the concept should not shape the current implementation.

## Current Project Context

`rustos` currently prioritizes:

- minimalism
- educational clarity
- UEFI-first boot flow
- direct QEMU workflow
- explicit module boundaries
- bounded emulator testing
- a documentation-first Unix-like direction

Because of that, `blog_os` material should be filtered through these project rules.

## Chapter-by-Chapter Policy

## Bare Bones

### A Freestanding Rust Binary
- Category: **Adopt**
- Why:
  - foundational for kernel work
  - aligns directly with `no_std` kernel design
  - supports minimalism and clarity
- Notes:
  - keep the implementation aligned with the current UEFI-first setup

### A Minimal Rust Kernel
- Category: **Adopt**
- Why:
  - directly supports the current project foundation
  - reinforces small entrypoints and explicit boot flow
- Notes:
  - preserve the current `rustos` module boundaries instead of copying exact file layout

### VGA Text Mode
- Category: **Adapt**
- Why:
  - useful as a concept for early output
  - not the best primary output path for the current UEFI-first workflow
- Notes:
  - prefer UEFI console and current boot logging first
  - VGA text mode can be added later as an architecture-specific fallback or teaching module
  - do not make VGA the main logging path for the MVP

### Testing
- Category: **Adopt**
- Why:
  - strongly aligned with project goals
  - directly improves code quality and contributor confidence
  - helps keep future low-level work safe and incremental
- Notes:
  - adapt the testing approach to the current `xtask` and QEMU workflow
  - prefer a layered testing model:
    - unit tests for pure logic
    - bounded emulator tests for boot/runtime behavior

## Interrupts

### CPU Exceptions
- Category: **Adopt**
- Why:
  - essential low-level kernel concept
  - directly supports runtime correctness and debugging
- Notes:
  - implement only the smallest useful exception groundwork first
  - keep logs and failure behavior explicit
  - treat modeled exception readiness and installed exception handlers as different milestones
  - complete boot-mode selection for exception testing before claiming a real controlled exception path
  - consider the first exception milestone complete only when the success marker is emitted by the real handler path

### Double Faults
- Category: **Adapt**
- Why:
  - important for robustness and debugging
  - but easy to overcomplicate early
- Notes:
  - adopt the concept carefully
  - keep the implementation minimal and well-tested
  - do not introduce large exception infrastructure before the basic exception path is clear
  - defer double-fault completion claims until the first controlled breakpoint path is real, bounded, and easy to diagnose

### Hardware Interrupts
- Category: **Adopt**
- Why:
  - necessary for timer and input groundwork
  - directly supports future Unix-like runtime behavior
- Notes:
  - start with the smallest useful interrupt path
  - timer-first is likely the best initial direction

## Memory Management

### Introduction to Paging
- Category: **Adopt**
- Why:
  - essential conceptual foundation
  - strongly aligned with future kernel/user boundary work
- Notes:
  - use it as a conceptual and documentation reference first

### Paging Implementation
- Category: **Adapt**
- Why:
  - important for future memory and user-space direction
  - but must fit the current UEFI-first environment and current code structure
- Notes:
  - do not copy implementation details blindly
  - keep paging work incremental and testable

### Heap Allocation
- Category: **Adapt**
- Why:
  - useful only when the kernel actually needs dynamic allocation
  - easy to introduce too early
- Notes:
  - heap support should follow a clear need
  - do not add allocation just because the reference does

### Allocator Designs
- Category: **Defer**
- Why:
  - valuable later
  - too much design space for the current stage
- Notes:
  - first prove that allocation is needed
  - then choose the smallest allocator that fits the actual use case

## Multitasking

### Async/Await
- Category: **Defer**
- Why:
  - interesting and educational
  - too advanced for the current MVP path
  - task model and syscall direction should come first
- Notes:
  - do not let async shape the early kernel architecture
  - revisit only after:
    - memory groundwork
    - interrupt groundwork
    - task model direction
    - syscall direction
    are all clearer

## Summary Table

| Topic | Category | Short Reason |
|---|---|---|
| A Freestanding Rust Binary | Adopt | Core kernel foundation |
| A Minimal Rust Kernel | Adopt | Directly aligned with current design |
| VGA Text Mode | Adapt | Useful, but not primary for UEFI-first MVP |
| Testing | Adopt | Strong fit for project quality and workflow |
| CPU Exceptions | Adopt | Essential runtime groundwork |
| Double Faults | Adapt | Important, but should stay minimal |
| Hardware Interrupts | Adopt | Needed for timer and runtime growth |
| Introduction to Paging | Adopt | Essential conceptual foundation |
| Paging Implementation | Adapt | Important, but must fit current architecture |
| Heap Allocation | Adapt | Only when justified by real need |
| Allocator Designs | Defer | Too early before real allocation need |
| Async/Await | Defer | Too advanced for current MVP |

## Practical Rules for Contributors

When using `blog_os` as a reference:

1. Start from the current `rustos` milestone, not from the `blog_os` chapter order.
2. Prefer the smallest implementation that proves the concept.
3. Preserve current project boundaries unless there is a strong reason to change them.
4. Keep UEFI-first assumptions in mind.
5. Add tests whenever logic can be tested without emulation.
6. Use emulator tests only for behavior that truly requires runtime validation.
7. Document why a `blog_os` idea was adopted, adapted, or deferred if the choice is not obvious.
8. Do not treat a post-trigger log message as proof of a real exception handler path.
9. For exception milestones, require explicit boot-mode selection, real handler installation, and handler-originated success markers before marking the milestone complete.

## Testing Guidance from `blog_os`

`blog_os` is especially valuable as a testing reference.

For `rustos`, the preferred testing interpretation is:

- use unit tests for pure logic first
- use bounded QEMU tests for boot/runtime behavior
- keep emulator tests few and meaningful
- avoid making all validation depend on full-system boot tests

This means `blog_os` testing ideas should be adapted into the current `xtask` workflow rather than copied mechanically.

## What We Intentionally Avoid

At the current stage, `rustos` should avoid:

- copying `blog_os` structure chapter-by-chapter
- introducing BIOS-first assumptions into the main path
- replacing the current UEFI-first workflow without a strong reason
- adding subsystems only because they appear in the reference
- treating the reference as a completeness checklist

## Revisit Conditions

This policy should be revisited when:

- the boot strategy changes
- the target architecture changes
- the project begins real syscall work
- the project begins real task switching work
- the project begins real paging implementation
- the project needs allocation for concrete reasons
- the project scope expands beyond the current MVP

## Decision Summary

For `rustos`, `blog_os` is:

- a strong reference
- a selective guide
- a source of proven low-level ideas
- a source of testing inspiration

It is not:

- the project architecture
- the project roadmap
- the project compatibility target

`rustos` should continue to borrow from `blog_os` where it improves clarity and correctness, and diverge where minimalism, UEFI-first design, or Unix-like MVP goals require a different path.

## Related Documents

- `docs/roadmap.md`
- `docs/architecture.md`
- `docs/unix-like.md`

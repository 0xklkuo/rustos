# Testing Strategy

## Purpose

This document defines the testing strategy for `rustos`.

The project is a minimal, educational operating system built in Rust. Testing should reflect that purpose:

- keep tests simple
- keep tests explicit
- prefer fast feedback
- test pure logic without emulation when possible
- use QEMU only when the behavior depends on boot, firmware, or hardware-facing runtime behavior

The goal is not to maximize test count. The goal is to build confidence without hiding the system behind a large testing framework.

## Testing Principles

### 1. Prefer the smallest useful test

If a function can be tested as a normal unit test, do that first.

Do not use QEMU for logic that can be tested directly in Rust.

### 2. Keep emulator tests meaningful

QEMU tests are slower and more fragile than unit tests. They should validate only behavior that actually needs emulation, such as:

- boot success
- expected boot logs
- panic behavior
- exception behavior
- interrupt behavior

As exception and interrupt groundwork is introduced, emulator tests should stay narrow:
- one smoke test for the expected exception path
- one smoke test for the expected interrupt path
- clear success markers in output
- bounded execution with explicit timeout behavior

For the controlled exception path stage, prefer one deliberate and well-bounded exception first.
The recommended first exception is a breakpoint-style path because it is easier to trigger intentionally, easier to explain, and less risky than more destructive exception cases.

During the current scaffolded stage, the exception smoke test should boot the kernel in an explicit exception-test mode instead of assuming the normal boot path will trigger exception behavior automatically.

Once a real exception handler is installed, the exception-path success marker should be emitted by the handler itself, not by ordinary control flow after the trigger instruction.

### 3. Keep tests bounded

Automated test runs must not hang indefinitely.

All emulator-based tests should:
- have a timeout
- fail clearly
- report the expected success marker or failure condition

### 4. Test behavior, not implementation trivia

Tests should focus on:
- state transitions
- public behavior
- invariants
- explicit subsystem boundaries

Avoid tests that only mirror the implementation line by line.

### 5. Keep the testing model teachable

A contributor should be able to understand:
- what kinds of tests exist
- when to add each kind
- how to run them
- why a given test belongs in that layer

## Testing Layers

`rustos` uses a small testing pyramid.

### Layer 1 — Unit tests

Use normal Rust unit tests for pure logic.

These are the preferred default.

Good candidates:
- runtime state summaries
- memory state transitions
- frame allocator placeholder behavior
- heap strategy decisions
- syscall number decoding later
- descriptor table logic later
- path parsing helpers later

Properties:
- fast
- deterministic
- easy to debug
- no emulator required

### Layer 2 — Kernel logic tests

Use focused tests for `no_std`-friendly logic that still does not require full boot execution.

Examples:
- address alignment helpers
- page range helpers
- frame bookkeeping
- task state transitions
- descriptor ownership rules

These should still stay small and explicit.

### Layer 3 — QEMU smoke tests

Use bounded QEMU tests for behavior that depends on the boot/runtime environment.

Examples:
- kernel boots successfully
- expected boot messages appear
- runtime initialization sequence appears
- panic path is visible
- exception path works when introduced
- interrupt path works when introduced

These tests should be few and meaningful.

For exception and interrupt groundwork, prefer smoke tests that answer one question each:
- does the expected exception handler path run?
- does the expected interrupt initialization path run?
- does the expected success marker appear before timeout?

For the controlled exception path stage, the exception smoke test should answer an even narrower question:
- does the kernel enter an explicit exception-test boot mode?
- can the kernel trigger one deliberate exception safely?
- does the real breakpoint handler run?
- does the handler-originated success marker appear?
- does the system remain bounded and diagnosable?

Avoid combining many low-level behaviors into one emulator test.

### Layer 4 — Negative-path emulator tests

Use emulator tests for deliberate failure paths only when the subsystem exists.

Examples:
- breakpoint exception
- page fault
- double fault strategy validation
- invalid syscall handling later

These should be added carefully and only when the corresponding subsystem is real.

For exception and interrupt work, add negative-path tests gradually:
- start with a breakpoint or other controlled exception smoke test
- add double-fault validation only after the basic exception path is stable
- add interrupt-specific negative tests only when the interrupt subsystem has a clear success path and failure model

The controlled exception path should be treated as a bridge between basic runtime logging and more advanced exception handling work.
It should stay small enough that a failure points to one subsystem first.

## What to Test First

The current project already has several good unit-test targets.

### Current pure logic candidates

These should be tested first:

- `arch::RuntimeState`
- `arch::runtime_summary`
- `console::State`
- `console::state_summary`
- `memory::State`
- `memory::HeapStrategy`
- `memory::FrameAllocator`
- `memory::state_summary`

These tests are valuable because they:
- are fast
- are stable
- improve discipline
- do not require firmware or QEMU

To keep these tests easy to run on the host, pure logic should gradually move into a small host-testable crate such as `nucleus` instead of staying tightly coupled to the UEFI-facing kernel crate.

That host-testable crate should contain:
- runtime state types
- summary helpers
- memory bookkeeping helpers
- frame allocator placeholder logic
- other pure logic that does not require firmware or hardware-facing setup

## Test Placement

### Unit tests

Prefer inline unit tests with `#[cfg(test)]` for small pure logic modules.

This works well when:
- the tested logic is local to one module
- the tests are short
- the tests help explain the module behavior

### Separate test modules

Use separate test modules or files when:
- the test setup is larger
- multiple modules are involved
- the test would distract from the main implementation

### Emulator tests

Emulator tests should be driven through the project workflow commands, not hidden behind ad hoc shell scripts.

## Standard Commands

The project should expose a small, consistent test workflow.

Recommended commands:

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`
- `cargo run -p xtask -- test-unit`
- `cargo run -p xtask -- test-qemu`

`test-unit` should prefer host-side tests for crates that contain pure logic, such as a future `nucleus` crate, instead of trying to force all kernel-adjacent tests through the UEFI-facing runtime crate.

If a combined local validation command exists, it should call these in a clear order.

## CI Strategy

CI should separate fast checks from emulator checks.

### Fast CI checks
These should run on every pull request:

- formatting
- linting
- workspace checks
- unit tests

### Emulator CI checks
These should run as bounded smoke tests:

- boot test
- runtime log validation
- later: exception smoke tests
- later: interrupt smoke tests

This split keeps feedback fast while still validating the real boot path.

When exception and interrupt smoke tests are added, keep them separate from the basic boot smoke test whenever practical. This makes failures easier to localize and keeps CI output easier to understand.

## When to Add a Unit Test

Add a unit test when:
- a module has state transitions
- a helper has non-trivial logic
- a summary or decision function can regress silently
- a bug was fixed and should stay fixed

## When to Add a QEMU Test

Add a QEMU test when:
- the behavior depends on firmware or boot
- the behavior depends on CPU/runtime state
- the behavior cannot be validated meaningfully as a unit test
- the test validates a real milestone boundary

For exception and interrupt groundwork, add a QEMU test only when:
- the handler or initialization path is visible in logs or another explicit success marker
- the test can be bounded reliably
- the behavior is narrow enough that a failure points to one subsystem first

For the controlled exception path stage, the preferred success marker should be a plain-language exception log that clearly distinguishes:
- exception-test boot mode entered
- exception trigger start
- real breakpoint handler reached
- exception handling complete or expected stop condition

The success marker should come from the handler path itself.

## What to Avoid

Avoid:
- large test frameworks
- hidden test magic
- emulator tests for pure logic
- brittle output matching when a smaller invariant would work
- adding many tests for placeholder code with no real behavior
- claiming coverage quality from test count alone

## Success Criteria for the Testing Strategy

The testing strategy is working if:

- contributors know which test type to add
- unit tests cover pure logic first
- emulator tests stay bounded
- CI remains understandable
- failures are easy to localize
- the project stays small and teachable

## Near-Term Testing Roadmap

### Immediate next steps
- add unit tests for current pure logic modules
- split local test workflow into unit and QEMU commands
- align CI with the same split
- extract host-testable pure logic into a small crate such as `nucleus`
- make exception-test boot mode explicit in the kernel and test workflow
- require handler-originated success markers for the first real exception path

### Later steps
- add exception smoke tests
- add interrupt smoke tests
- add memory bookkeeping tests
- add syscall and task model tests when those subsystems exist

### Controlled exception path rollout
A practical rollout order is:
1. boot smoke test
2. runtime initialization smoke test
3. controlled exception smoke test
4. interrupt initialization smoke test
5. double-fault or other negative-path validation only after the basic paths are stable

This order keeps the low-level test surface understandable and avoids introducing many fragile emulator tests at once.

## Summary

`rustos` should use a simple testing model:

- unit tests first
- emulator tests only when needed
- bounded execution always
- explicit workflows
- minimal tooling
- educational clarity over test complexity

This keeps the project reliable without making the testing system larger than the kernel itself.

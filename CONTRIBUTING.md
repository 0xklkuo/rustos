# Contributing to rustos

Thanks for your interest in `rustos`.

`rustos` is a minimal, educational operating system project. Contributions are welcome when they make the codebase clearer, stronger, and easier to evolve without bloating it.

## Read this first

Before starting work, read:

1. `README.md`
2. `docs/spec.md`
3. `docs/architecture.md`
4. `docs/roadmap.md`

Those four docs are the main project contract.

## Contribution rules

When contributing, prefer the following:

- keep changes small and focused
- prefer readability over abstraction
- fix root causes instead of adding surface patches
- keep firmware-facing code narrow
- move host-testable logic into `nucleus/` when practical
- avoid new dependencies unless they solve a real problem
- update docs when behavior or project decisions change

## What usually makes a good contribution

- documentation clarifications
- small focused bug fixes
- better tests or validation
- workflow improvements
- small subsystem refinements that reduce complexity
- code cleanup that strengthens boundaries without rewriting the project

## What usually does not fit well

- speculative frameworks
- large rewrites without a concrete need
- feature growth that outruns the current milestone
- production-oriented complexity the project does not claim to support

## Validation

Before submitting a change, run the smallest relevant validation set.

Common commands:

- `cargo run -p xtask -- check`
- `cargo run -p xtask -- fmt`
- `cargo run -p xtask -- lint`
- `cargo run -p xtask -- test-unit`

If your change affects the boot or runtime path, also run:

- `cargo run -p xtask -- test-qemu`

If your change affects the controlled exception path, also run:

- `cargo run -p xtask -- test-exception`

## Pull request expectations

A good pull request should explain:

- what changed
- why it changed
- any important tradeoffs
- how it was validated

Prefer small pull requests over broad mixed changes.

## Safety and `unsafe`

This project contains low-level code and `unsafe` Rust.

If you add or change `unsafe` code, document:

- why it is needed
- what assumptions it relies on
- what must stay true for it to remain correct

## Commit style

Conventional commits are preferred when practical.

Examples:

- `docs: simplify project contract`
- `refactor: split nucleus into subsystem modules`
- `feat: add minimal vfs starter`
- `fix: tighten memory boundary wording`

## If you are unsure where to start

A good first contribution is usually one of:

- a docs cleanup
- a test improvement
- a small boundary clarification
- a focused cleanup in `nucleus/` or `xtask/`

When in doubt, ask a focused question before starting a large change.

# Contributing to rustos

Thanks for your interest in contributing to `rustos`.

`rustos` is a minimal, educational, and maintainable open-source project for learning Rust systems programming and modern operating system fundamentals. The project values clarity over cleverness and small, well-explained steps over large, complex changes.

## Project Goals

- Teach core OS concepts through a small Rust codebase
- Keep the design minimal and easy to understand
- Follow modern Rust best practices
- Make the project approachable for contributors of different experience levels
- Prefer explicit design decisions over hidden complexity

## Non-Goals

At least for the MVP, `rustos` does not aim to be:

- a production operating system
- a feature-complete Unix clone
- a highly optimized kernel at the cost of readability
- a place for large speculative subsystems without clear teaching value

## Ways to Contribute

You can help by contributing:

- documentation improvements
- typo fixes and wording clarifications
- bug reports
- small focused code changes
- tests and validation improvements
- build and tooling improvements
- design feedback grounded in the project goals

## Before You Start

Please:

1. Read the `README.md`
2. Review the docs in `docs/`
3. Check open issues and existing discussions
4. Prefer small, focused pull requests
5. Make sure your change matches the project goals

If your change is large, architectural, or changes project direction, open an issue or discussion first.

## Development Principles

When contributing, follow these principles:

- keep changes minimal and focused
- prefer readability over abstraction
- avoid over-engineering
- isolate `unsafe` code and document its invariants
- use simple names
- keep modules small and responsibilities clear
- explain why a design exists when it is not obvious
- do not introduce dependencies without strong justification

## Rust Style

This project follows the official Rust style guide.

General expectations:

- run formatting before submitting changes
- keep code idiomatic and consistent
- avoid unnecessary macros
- avoid unnecessary generics or traits in early-stage code
- prefer explicit control flow when it improves understanding

## Commit Style

Use conventional commits when possible.

Examples:

- `chore: initialize workspace and project foundation`
- `docs: clarify roadmap scope`
- `feat: add minimal kernel entry point`
- `fix: handle panic output during early boot`

## Pull Request Guidelines

A good pull request should be:

- small enough to review comfortably
- clearly scoped
- explained in plain language
- linked to an issue when relevant

Please include:

- what changed
- why it changed
- any tradeoffs or limitations
- how you validated it

## Code Review Expectations

Reviews will focus on:

- correctness
- simplicity
- maintainability
- educational value
- consistency with project goals

A change may be rejected if it:

- adds complexity without clear benefit
- introduces abstractions too early
- makes the code harder to teach or understand
- expands scope beyond the current milestone

## Reporting Bugs

When reporting a bug, include:

- what you expected
- what happened instead
- steps to reproduce
- host environment details
- toolchain details
- logs or screenshots if useful

If the issue is related to booting or emulation, include as much exact output as possible.

## Suggesting Features

Feature requests are welcome, but they should be grounded in the project goals.

Please explain:

- the problem being solved
- why it belongs in `rustos`
- whether it fits the current milestone
- whether a smaller version of the idea would work

## Documentation Contributions

Documentation is a first-class contribution.

Good documentation changes:

- reduce ambiguity
- explain intent clearly
- help new contributors get started faster
- keep wording concise and direct

## Testing and Validation

Before submitting, validate your change as much as practical.

At minimum:

- ensure the project still builds
- ensure formatting is correct
- ensure any changed docs are accurate

As the project grows, more validation steps may be added.

## Security and Safety

This project includes low-level systems code. Be careful with:

- `unsafe` blocks
- memory assumptions
- architecture-specific behavior
- boot and runtime invariants

If you add `unsafe` code, document:

- why it is needed
- what assumptions it relies on
- what must remain true for it to be correct

## Communication

Please keep communication:

- respectful
- direct
- constructive
- focused on the code and project goals

When in doubt, choose clarity.

## License

By contributing to `rustos`, you agree that your contributions will be licensed under the MIT License.

## Questions

If something is unclear, ask before making a large change.

Clear questions early are better than large rewrites later.

# Support

Thanks for your interest in `rustos`.

`rustos` is a small, educational operating system project. Support is focused on helping contributors and learners understand the project, use the documented workflow, and report clear issues.

## Before Asking for Help

Please check these first:

1. `README.md`
2. `docs/README.md`
3. `docs/architecture.md`
4. `docs/roadmap.md`
5. `docs/testing.md`
6. `CONTRIBUTING.md`

Many common questions about scope, workflow, and current project status are already answered there.

## Where to Ask Questions

### GitHub Discussions
Use GitHub Discussions for:

- setup questions
- workflow questions
- architecture questions
- design feedback
- learning-oriented questions
- open-ended ideas that are not yet concrete bugs or feature requests

This is the best place for:

- “How do I run this?”
- “Why was this design chosen?”
- “Does this idea fit the project goals?”
- “What should I read before contributing?”

### GitHub Issues
Use GitHub Issues for:

- reproducible bugs
- documentation mistakes
- focused feature requests
- clearly scoped improvement proposals

A good issue should explain:

- what you expected
- what happened instead
- how to reproduce it
- your environment
- any relevant logs or screenshots

## What to Include in a Support Request

When asking for help, include as much relevant detail as practical.

### For setup or workflow help
Include:

- host operating system
- host architecture
- Rust toolchain version
- QEMU version
- firmware setup details if relevant
- the command you ran
- the full error output

### For boot or emulator issues
Include:

- the exact command you ran
- whether you used the standard `xtask` workflow
- the full boot log or failure output
- whether the issue happens consistently
- any local firmware overrides you used

### For design or contribution questions
Include:

- the problem you are trying to solve
- why you think it belongs in `rustos`
- whether it fits the current roadmap
- whether a smaller version of the idea would work

## Support Scope

Support is best-effort.

Because `rustos` is an educational and early-stage project, support is mainly intended for:

- understanding the documented workflow
- clarifying project scope and direction
- helping contributors make focused changes
- identifying real bugs or documentation gaps

Support is not intended to provide:

- guaranteed response times
- private consulting
- broad platform support beyond the documented target and workflow
- help for heavily modified local forks without enough reproduction detail

## Project Scope Reminder

`rustos` is intentionally minimal.

Please keep support requests aligned with the project goals:

- educational value
- clarity
- minimalism
- maintainability
- explicit design boundaries

Requests that assume production readiness, broad hardware support, or full Unix compatibility are likely outside the current scope.

## Security Issues

Please do not report security-sensitive issues through public issues if the report would expose a serious vulnerability or unsafe disclosure.

Instead, follow the guidance in `SECURITY.md`.

## Contributing Help

If you want to contribute but are unsure where to start:

1. read `CONTRIBUTING.md`
2. review `docs/roadmap.md`
3. look for small documentation or cleanup tasks first
4. ask a focused question in Discussions before starting a large change

Small, clear questions early are better than large speculative changes later.

## Response Expectations

Responses may vary depending on maintainer availability.

In general, the most helpful requests are:

- specific
- reproducible
- scoped to one problem
- grounded in the current project goals
- written in clear plain language

## Summary

Use:

- Discussions for questions and design conversation
- Issues for reproducible bugs and focused improvements

Before asking for help, read the main project docs and include enough detail for someone else to understand the problem quickly.

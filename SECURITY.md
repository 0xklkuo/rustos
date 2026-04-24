# Security Policy

## Purpose

`rustos` is a minimal, educational, early-stage operating system project.

This project includes low-level systems code, boot code, architecture-specific behavior, and `unsafe` Rust. Because of that, security-related issues should still be reported responsibly, even though the project is not intended for production use.

## Supported Versions

At this stage, only the latest state of the default branch is considered supported for security review.

| Version | Supported |
|---|---|
| `main` | Yes |
| tagged releases before the latest release | No |
| older commits and experimental branches | No |

## Security Scope

Please keep the current project scope in mind:

- `rustos` is an educational project
- it is not a production operating system
- it is not intended for deployment on real systems
- it does not currently provide a stable security boundary
- many subsystems are intentionally incomplete

That means some issues may be accepted as valid engineering or safety concerns without being treated as production-grade vulnerabilities.

Examples of issues that are still useful to report:

- unsound `unsafe` code
- memory safety issues
- incorrect low-level assumptions
- privilege-boundary mistakes once such boundaries exist
- boot-path trust or integrity issues
- unsafe parsing or unchecked input in tooling
- accidental exposure of secrets in repository files or workflows
- CI or release-process weaknesses that could affect contributors

Examples of issues that are currently out of scope or lower priority:

- missing hardening features that the project does not claim to provide
- lack of production-grade isolation in unfinished subsystems
- theoretical attacks that depend on unsupported deployment assumptions
- issues in old unsupported commits or branches

## How to Report a Security Issue

Please do **not** open a public issue for a suspected security problem.

Instead, use GitHub private vulnerability reporting for this repository when it is available.

Please include:

- a clear summary of the issue
- affected files, modules, or workflows
- steps to reproduce, if applicable
- impact and assumptions
- logs, screenshots, or proof of concept if useful
- any suggested fix or mitigation, if you have one

Please keep reports concise, factual, and reproducible.

## What to Expect

Because this is a small educational project, response times may vary.

The general process is:

1. acknowledge the report
2. review and reproduce the issue
3. decide whether it is in scope
4. prepare a fix or mitigation if appropriate
5. disclose the fix publicly after it is ready

Not every report will result in a CVE, advisory, or urgent patch. Some reports may instead lead to:

- documentation updates
- clearer safety comments
- tighter invariants
- workflow hardening
- design changes in a later milestone

## Disclosure Guidance

Please avoid public disclosure until the issue has been reviewed.

Responsible private reporting helps protect:

- contributors
- users experimenting with the project
- the integrity of the repository and release process

Once a fix is available, the issue can be documented publicly in a normal project update, release note, or changelog entry.

## Security Philosophy

For `rustos`, security work should follow the same project principles as the rest of the codebase:

- keep boundaries explicit
- keep `unsafe` code narrow and documented
- prefer simple and reviewable designs
- avoid claiming security properties that do not exist yet
- treat correctness and soundness as first-class concerns

## Notes

If you are unsure whether something is a security issue, it is still fine to report it privately.

Clear reports are welcome, especially when they help improve:

- soundness
- safety
- contributor trust
- release hygiene
- documentation accuracy

# ADR 0001: MVP target platform

- Status: accepted
- Date: 2026-04-13

## Context

`rustos` is a minimal, educational, maintainable open-source project for learning Rust systems programming and operating system fundamentals.

The project is being developed primarily on Apple Silicon Macs. The MVP should be easy to understand, easy to run locally, and aligned with strong existing learning material in the Rust ecosystem.

Two main target directions were considered for the MVP:

1. `x86_64-unknown-uefi`
2. `aarch64` with UEFI

## Decision

For the MVP, `rustos` will target `x86_64-unknown-uefi`.

Development will be performed on Apple Silicon hosts, and the OS will be run locally through QEMU.

## Rationale

### Why `x86_64-unknown-uefi`

- It has stronger educational support in the Rust OS ecosystem.
- It aligns better with existing references such as `blog_os` and related learning resources.
- It keeps the early project simpler and more teachable.
- It allows the project to focus on OS fundamentals before introducing additional architecture-specific complexity.
- It remains practical for Apple Silicon developers because QEMU can emulate the target platform.

### Why not `aarch64` first

- It would better match the host CPU architecture, but it introduces more platform-specific complexity earlier.
- It has fewer beginner-friendly references and examples for a minimal educational OS path.
- It would make the first milestones harder to keep minimal and contributor-friendly.

### Why UEFI

- It is a modern boot path.
- It is cleaner than starting with legacy BIOS.
- It fits the project's goal of teaching modern systems development practices.

### Why QEMU

- It is the most practical local execution environment across host platforms.
- It supports repeatable development workflows.
- It avoids tying the project too early to physical hardware or host-specific setup.

## Consequences

### Positive

- The MVP path is clearer and easier to document.
- The project can reuse well-understood patterns from the Rust OS learning ecosystem.
- Contributors are more likely to find familiar references and debugging approaches.
- The repo can prioritize clarity and teaching value over early multi-architecture support.

### Negative

- The target architecture does not match the primary host architecture.
- Local execution on Apple Silicon depends on emulation.
- Future `aarch64` support will require explicit design work rather than falling out naturally from the MVP.

## Non-Goals for This Decision

This decision does not commit the project to:

- permanent `x86_64` exclusivity
- BIOS support in the MVP
- multi-architecture support in early milestones
- hardware-first development outside emulation

## Revisit Conditions

This decision should be revisited when one or more of the following become true:

- the MVP is stable and documented
- the project has a clear architecture abstraction boundary
- there is demand for `aarch64` support
- the educational material for `aarch64` support is ready to stay consistent with project quality goals

## Related Documents

- `docs/roadmap.md`
- `docs/architecture.md`
- `README.md`

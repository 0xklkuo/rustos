# Changelog

All notable changes to `rustos` will be documented in this file.

This project is still in an early stage. The changelog is intended to make public releases, milestone consolidation, and contributor-facing changes easier to follow.

The format is intentionally simple and release-oriented.

## [Unreleased]

### Added
- documentation map in `docs/README.md`
- dedicated direction notes for:
  - paging
  - syscalls
  - tasks
  - descriptors
- host-testable pure logic in `nucleus/` for:
  - interrupt state
  - memory bookkeeping
  - paging helpers
  - syscall models and tiny dispatch sketch
  - task models
  - descriptor models and tiny ownership sketch
- kernel-facing subsystem boundaries for:
  - memory
  - paging
  - syscall
- direct `xtask` workflows for:
  - `check`
  - `fmt`
  - `lint`
  - `test-unit`
  - `test-qemu`
  - `test-exception`
  - `test`
  - `run`
- bounded QEMU smoke testing
- controlled breakpoint exception smoke testing
- real UEFI memory-map discovery
- minimal paging direction and architecture-facing paging probe
- minimal Unix-like boundary starter for:
  - syscalls
  - tasks
  - descriptor-like handles

### Changed
- consolidated documentation ownership and structure
- trimmed `README.md` into a clearer landing page and quick start
- made `docs/roadmap.md` the single source of truth for project status
- refocused `docs/architecture.md` on stable principles and repository structure
- reduced repeated milestone/status wording across subsystem docs
- improved source-level documentation across kernel, shared logic, and workflow code
- clarified public API docs for memory, paging, interrupt, syscall, and console boundaries
- improved contributor-facing source documentation for `xtask`

### Fixed
- corrected stale and misleading documentation in memory and paging helpers
- clarified exception-handler wording to avoid overstating interrupt coverage
- fixed CI and lint issues introduced during paging and Unix-like boundary work
- aligned boot-log wording for paging and syscall boundary reporting

## [0.1.0-alpha.1] - Planned

First public release preparation for `rustos`.

This release is intended to mark the project as publicly launch-ready on GitHub while keeping expectations clear: `rustos` is still an educational, early-stage operating system project.

### Highlights
- minimal bootable Rust UEFI kernel foundation
- direct QEMU workflow from an Apple Silicon macOS host
- bounded emulator smoke tests and host-side unit tests
- real breakpoint-first exception path
- real UEFI memory-map discovery
- minimal paging direction boundary
- minimal Unix-like boundary starter
- consolidated contributor-facing documentation

### What Works
- building the workspace with the documented local workflow
- running formatting, linting, and host-side unit tests through `xtask`
- booting the kernel in QEMU
- validating the normal boot path with a bounded QEMU smoke test
- validating the controlled breakpoint path with a bounded exception smoke test

### Known Limitations
- no user-mode execution
- no real syscall ABI wiring
- no scheduler or multitasking
- no filesystem implementation
- no networking
- no descriptor tables
- no heap allocator
- no page-table management
- no POSIX compatibility claims

### Notes
- nightly Rust is currently required for the real x86_64 breakpoint-handler path because the `x86-interrupt` ABI is still unstable
- the project remains intentionally minimal and educational
- future releases should continue to prefer small, explicit, well-documented steps

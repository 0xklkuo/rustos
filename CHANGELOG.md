# Changelog

All notable changes to `rustos` will be documented in this file.

## [Unreleased]

### Added
- `docs/spec.md` as the main technical contract for the current implementation
- a minimal VFS starter in `nucleus::vfs` and `kernel::vfs`
- host-testable path validation and starter namespace coverage for `/`, `/dev`, `/dev/console`, and `/init`

### Changed
- split `nucleus` from one oversized `lib.rs` into per-subsystem source files
- consolidated documentation around `README.md`, `docs/spec.md`, `docs/architecture.md`, and `docs/roadmap.md`
- trimmed duplicated kernel-side tests that were repeating `nucleus` coverage
- simplified contributor-facing docs so the main project contract is easier to find and maintain
- made `xtask check` validate the UEFI kernel target directly so target-specific regressions surface earlier

### Removed
- redundant direction documents that duplicated the new core docs

## [0.1.0-alpha.1] - Planned

First public release preparation for `rustos`.

### Highlights
- minimal bootable Rust UEFI kernel foundation
- direct QEMU workflow from an Apple Silicon macOS host
- bounded emulator smoke tests and host-side unit tests
- real breakpoint-first exception path
- real UEFI memory-map discovery
- minimal paging direction boundary
- minimal Unix-like boundary starter
- consolidated contributor-facing documentation

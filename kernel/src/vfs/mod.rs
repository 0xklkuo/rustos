//! Virtual-filesystem boundary for `rustos`.
//!
//! This module keeps the kernel-facing VFS starter small. The host-testable
//! path and node model lives in `nucleus`, while this module exposes the first
//! kernel-visible namespace readiness checks.

pub use nucleus::vfs::{LookupError, Node, NodeKind, Path, lookup, lookup_summary, node_summary};

/// Boot-time log marker for the VFS boundary.
pub const INIT_MESSAGE: &str = "rustos: vfs init";

/// Small kernel-side VFS initialization result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitResult {
    namespace_ready: bool,
    console_path_ready: bool,
}

impl InitResult {
    /// Creates a new VFS initialization result.
    #[must_use]
    pub const fn new(namespace_ready: bool, console_path_ready: bool) -> Self {
        Self {
            namespace_ready,
            console_path_ready,
        }
    }

    /// Returns whether the tiny namespace boundary is ready.
    #[must_use]
    pub const fn is_namespace_ready(self) -> bool {
        self.namespace_ready
    }

    /// Returns whether the console device path is present.
    #[must_use]
    pub const fn is_console_path_ready(self) -> bool {
        self.console_path_ready
    }
}

/// Performs the current VFS initialization step.
#[must_use]
pub fn init() -> InitResult {
    let namespace_ready = matches!(lookup(Path::new("/")), Ok(node) if node.kind() == NodeKind::Directory)
        && matches!(lookup(Path::new("/dev")), Ok(node) if node.kind() == NodeKind::Directory);
    let console_path_ready = matches!(
        lookup(Path::new("/dev/console")),
        Ok(node) if node.kind() == NodeKind::Device
    );

    InitResult::new(namespace_ready, console_path_ready)
}

/// Returns a small plain-language summary of namespace readiness.
#[must_use]
pub const fn init_summary(result: InitResult) -> &'static str {
    if result.is_namespace_ready() {
        "rustos: vfs namespace ready"
    } else {
        "rustos: vfs deferred"
    }
}

/// Returns a small plain-language summary of console device-path readiness.
#[must_use]
pub const fn console_path_summary(result: InitResult) -> &'static str {
    if result.is_console_path_ready() {
        "rustos: vfs console path ready"
    } else {
        "rustos: vfs console path deferred"
    }
}

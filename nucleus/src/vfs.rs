//! Minimal virtual-filesystem direction that is safe to test on the host.
//!
//! This module keeps the first namespace boundary small and explicit. It does
//! not implement a real VFS, open file table, or dynamic path resolution.

/// Small classification of namespace nodes for the current stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// A namespace container such as `/` or `/dev`.
    Directory,
    /// A regular file-like node.
    File,
    /// A device-backed node.
    Device,
}

/// Small summary of one VFS node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    kind: NodeKind,
    writable: bool,
}

/// Small borrowed path wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Path<'a> {
    raw: &'a str,
}

/// Small error set for the current path and lookup rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupError {
    /// The provided path is empty.
    Empty,
    /// The provided path is not absolute.
    NotAbsolute,
    /// The provided path contains an empty component.
    EmptyComponent,
    /// The provided path does not resolve in the tiny starter namespace.
    NotFound,
}

impl Node {
    /// Creates a directory node.
    #[must_use]
    pub const fn directory() -> Self {
        Self {
            kind: NodeKind::Directory,
            writable: false,
        }
    }

    /// Creates a file node.
    #[must_use]
    pub const fn file(writable: bool) -> Self {
        Self {
            kind: NodeKind::File,
            writable,
        }
    }

    /// Creates a device node.
    #[must_use]
    pub const fn device(writable: bool) -> Self {
        Self {
            kind: NodeKind::Device,
            writable,
        }
    }

    /// Returns the node kind.
    #[must_use]
    pub const fn kind(self) -> NodeKind {
        self.kind
    }

    /// Returns whether the node supports writes in the current model.
    #[must_use]
    pub const fn is_writable(self) -> bool {
        self.writable
    }
}

impl<'a> Path<'a> {
    /// Creates a new borrowed path wrapper.
    #[must_use]
    pub const fn new(raw: &'a str) -> Self {
        Self { raw }
    }

    /// Returns the raw path string.
    #[must_use]
    pub const fn raw(self) -> &'a str {
        self.raw
    }

    /// Returns whether the path is absolute.
    #[must_use]
    pub fn is_absolute(self) -> bool {
        self.raw.starts_with('/')
    }

    /// Returns whether the path is the namespace root.
    #[must_use]
    pub fn is_root(self) -> bool {
        self.raw == "/"
    }

    /// Returns the number of non-empty path components.
    #[must_use]
    pub fn component_count(self) -> usize {
        self.raw
            .split('/')
            .filter(|component| !component.is_empty())
            .count()
    }

    /// Validates the path against the current tiny VFS rules.
    pub fn validate(self) -> Result<(), LookupError> {
        validate_path(self.raw)
    }
}

/// Validates a path against the current tiny VFS rules.
pub fn validate_path(raw: &str) -> Result<(), LookupError> {
    if raw.is_empty() {
        return Err(LookupError::Empty);
    }

    if !raw.starts_with('/') {
        return Err(LookupError::NotAbsolute);
    }

    if raw != "/" && raw.split('/').skip(1).any(str::is_empty) {
        return Err(LookupError::EmptyComponent);
    }

    Ok(())
}

/// Looks up a node in the tiny starter namespace.
pub fn lookup(path: Path<'_>) -> Result<Node, LookupError> {
    path.validate()?;

    match path.raw() {
        "/" | "/dev" => Ok(Node::directory()),
        "/dev/console" => Ok(Node::device(true)),
        "/init" => Ok(Node::file(false)),
        _ => Err(LookupError::NotFound),
    }
}

/// Returns a small plain-language summary of a node.
#[must_use]
pub const fn node_summary(node: Node) -> &'static str {
    match node.kind() {
        NodeKind::Directory => "rustos: vfs namespace ready",
        NodeKind::File => "rustos: vfs file path ready",
        NodeKind::Device => "rustos: vfs device path ready",
    }
}

/// Returns a small plain-language summary of a lookup result.
#[must_use]
pub fn lookup_summary(result: Result<Node, LookupError>) -> &'static str {
    match result {
        Ok(node) => node_summary(node),
        Err(LookupError::NotFound) => "rustos: vfs path not found",
        Err(_) => "rustos: vfs path invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LookupError, Node, NodeKind, Path, lookup, lookup_summary, node_summary, validate_path,
    };

    #[test]
    fn root_path_is_valid_and_has_no_components() {
        let path = Path::new("/");

        assert!(path.is_absolute());
        assert!(path.is_root());
        assert_eq!(path.component_count(), 0);
        assert_eq!(path.validate(), Ok(()));
    }

    #[test]
    fn console_path_is_valid_and_has_two_components() {
        let path = Path::new("/dev/console");

        assert!(path.is_absolute());
        assert!(!path.is_root());
        assert_eq!(path.component_count(), 2);
        assert_eq!(path.validate(), Ok(()));
    }

    #[test]
    fn invalid_paths_are_rejected() {
        assert_eq!(validate_path(""), Err(LookupError::Empty));
        assert_eq!(validate_path("dev/console"), Err(LookupError::NotAbsolute));
        assert_eq!(
            validate_path("/dev//console"),
            Err(LookupError::EmptyComponent)
        );
        assert_eq!(validate_path("/dev/"), Err(LookupError::EmptyComponent));
    }

    #[test]
    fn lookup_returns_expected_namespace_nodes() {
        assert_eq!(lookup(Path::new("/")), Ok(Node::directory()));
        assert_eq!(lookup(Path::new("/dev")), Ok(Node::directory()));
        assert_eq!(lookup(Path::new("/init")), Ok(Node::file(false)));
        assert_eq!(lookup(Path::new("/dev/console")), Ok(Node::device(true)));
    }

    #[test]
    fn unknown_paths_report_not_found() {
        assert_eq!(lookup(Path::new("/tmp")), Err(LookupError::NotFound));
        assert_eq!(
            lookup_summary(lookup(Path::new("/tmp"))),
            "rustos: vfs path not found"
        );
    }

    #[test]
    fn lookup_summaries_match_node_kind() {
        assert_eq!(
            node_summary(Node::directory()),
            "rustos: vfs namespace ready"
        );
        assert_eq!(
            node_summary(Node::file(false)),
            "rustos: vfs file path ready"
        );
        assert_eq!(
            node_summary(Node::device(true)),
            "rustos: vfs device path ready"
        );
    }

    #[test]
    fn lookup_exposes_console_as_writable_device() {
        let node = lookup(Path::new("/dev/console")).expect("console path should exist");

        assert_eq!(node.kind(), NodeKind::Device);
        assert!(node.is_writable());
        assert_eq!(lookup_summary(Ok(node)), "rustos: vfs device path ready");
    }
}

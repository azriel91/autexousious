use std::path::PathBuf;

use derive_new::new;

/// Namespace and its directory path.
#[derive(Clone, Debug, PartialEq, PartialOrd, new)]
pub struct NamespaceDirectory {
    /// Namespace, e.g. "test", "default", "user1".
    pub namespace: String,
    /// Path of the directory.
    pub path: PathBuf,
}

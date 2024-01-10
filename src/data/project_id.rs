//! Defines the project ID of the Firebase.

/// The Firebase project ID.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ProjectId {
    pub(crate) inner: String,
}

impl ProjectId {
    /// Creates a new project ID.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}

/// A photo URL of a user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct PhotoUrl {
    pub(crate) inner: String,
}

impl PhotoUrl {
    /// Creates a new photo URL.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}

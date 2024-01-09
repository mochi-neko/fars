#[derive(Clone, Debug)]
pub struct ApiKey {
    pub(crate) inner: String,
}

impl ApiKey {
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}

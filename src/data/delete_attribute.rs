/// Attributes to delete profile information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeleteAttribute {
    /// Delete the display name.
    DisplayName,
    /// Delete the photo url.
    PhotoUrl,
}

impl DeleteAttribute {
    /// Formats the delete attribute to a string representation of the Firebase Auth.
    pub fn format(self) -> &'static str {
        match self {
            | DeleteAttribute::DisplayName => "DISPLAY_NAME",
            | DeleteAttribute::PhotoUrl => "PHOTO_URL",
        }
    }
}

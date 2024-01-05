//! Defines the delete attribute of the Firebase Auth.

/// Attributes to delete profile information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeleteAttribute {
    /// Delete the display name.
    DisplayName,
    /// Delete the photo url.
    PhotoUrl,
}

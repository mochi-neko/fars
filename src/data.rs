//! Shared data structures for the Firebase Auth API.

// Private modules
mod delete_attribute;
mod idp_post_body;
mod provider_id;
mod provider_user_info;
mod user_data;

// Re-exports
pub use delete_attribute::DeleteAttribute;
pub use idp_post_body::IdpPostBody;
pub use provider_id::ProviderId;
pub use provider_user_info::ProviderUserInfo;
pub use user_data::UserData;

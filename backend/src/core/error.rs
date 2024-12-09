//! Top-level error types

use specta::Type;

#[derive(Type, Debug, thiserror::Error)]
#[error("An error occured while interacting with the system")]
pub enum AppError {
    Lock,
    Unexpected,
}

/// A suggestion displayed to the user
pub struct Suggestion(pub &'static str);

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

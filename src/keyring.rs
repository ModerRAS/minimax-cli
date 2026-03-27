#![allow(clippy::dead_code)]

//! Secure storage of API key using system keyring/credential store.
//!
//! Supports: Linux (Secret Service), macOS (Keychain), Windows (Credential Manager)

use keyring::Entry;
use std::error::Error;
use std::fmt;

/// Service name for keyring entry
const SERVICE: &str = "minimax-cli";
/// User name for keyring entry
const USER: &str = "api_key";

/// Errors that can occur when interacting with the keyring.
#[derive(Debug)]
pub enum KeyringError {
    /// No API key is stored in the keyring
    NotFound,
    /// Failed to store the API key
    SetFailed(String),
    /// Failed to retrieve the API key
    GetFailed(String),
    /// Failed to delete the API key
    DeleteFailed(String),
    /// Keyring is not available on this platform
    PlatformNotSupported(String),
}

impl fmt::Display for KeyringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyringError::NotFound => write!(f, "No API key found in keyring"),
            KeyringError::SetFailed(msg) => write!(f, "Failed to store API key: {}", msg),
            KeyringError::GetFailed(msg) => write!(f, "Failed to retrieve API key: {}", msg),
            KeyringError::DeleteFailed(msg) => write!(f, "Failed to delete API key: {}", msg),
            KeyringError::PlatformNotSupported(msg) => {
                write!(f, "Keyring not supported on this platform: {}", msg)
            }
        }
    }
}

impl Error for KeyringError {}

impl From<keyring::Error> for KeyringError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => KeyringError::NotFound,
            keyring::Error::PlatformFailure(msg) => KeyringError::SetFailed(msg.to_string()),
            keyring::Error::NoStorageAccess(msg) => KeyringError::GetFailed(msg.to_string()),
            keyring::Error::BadEncoding(_) => KeyringError::GetFailed("Bad encoding".to_string()),
            keyring::Error::TooLong(name, len) => {
                KeyringError::SetFailed(format!("{} exceeds {} chars", name, len))
            }
            keyring::Error::Invalid(name, reason) => {
                KeyringError::SetFailed(format!("{}: {}", name, reason))
            }
            keyring::Error::Ambiguous(_) => {
                KeyringError::GetFailed("Ambiguous credential".to_string())
            }
            _ => KeyringError::GetFailed("Unknown keyring error".to_string()),
        }
    }
}

/// Retrieve the API key from the system keyring.
///
/// # Returns
/// * `Ok(String)` - The stored API key
/// * `Err(KeyringError)` - If retrieval fails or no key is stored
///
/// # Example
/// ```ignore
/// match get_api_key() {
///     Ok(key) => println!("API key found: {}...", &key[..8.min(key.len())]),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_api_key() -> Result<String, KeyringError> {
    let entry = Entry::new(SERVICE, USER).map_err(|e| KeyringError::GetFailed(e.to_string()))?;
    entry.get_password().map_err(KeyringError::from)
}

/// Store the API key in the system keyring.
///
/// # Arguments
/// * `key` - The API key to store
///
/// # Returns
/// * `Ok(())` - Key stored successfully
/// * `Err(KeyringError)` - If storage fails
///
/// # Example
/// ```ignore
/// if let Err(e) = set_api_key("my-secret-api-key") {
///     eprintln!("Failed to store API key: {}", e);
/// }
/// ```
pub fn set_api_key(key: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(SERVICE, USER).map_err(|e| KeyringError::SetFailed(e.to_string()))?;
    entry.set_password(key).map_err(KeyringError::from)
}

/// Delete the API key from the system keyring.
///
/// # Returns
/// * `Ok(())` - Key deleted successfully or no key exists
/// * `Err(KeyringError)` - If deletion fails
///
/// # Example
/// ```ignore
/// if let Err(e) = delete_api_key() {
///     eprintln!("Failed to delete API key: {}", e);
/// }
/// ```
pub fn delete_api_key() -> Result<(), KeyringError> {
    let entry = Entry::new(SERVICE, USER).map_err(|e| KeyringError::DeleteFailed(e.to_string()))?;
    entry.delete_credential().map_err(KeyringError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require a real keyring on the system, so we skip them in CI
    // by checking for a CI environment variable.
    const CI_VAR: &str = "CI";

    fn is_ci() -> bool {
        std::env::var(CI_VAR).is_ok()
    }

    #[test]
    #[ignore = "requires keyring support, skipped on CI"]
    fn test_set_and_get_api_key() {
        if is_ci() {
            return;
        }

        let test_key = "test-api-key-12345";

        // Clean up any existing key first
        let _ = delete_api_key();

        // Set the key
        set_api_key(test_key).expect("should set API key");

        // Get the key
        let retrieved = get_api_key().expect("should get API key");
        assert_eq!(retrieved, test_key);

        // Clean up
        delete_api_key().expect("should delete API key");
    }

    #[test]
    #[ignore = "requires keyring support, skipped on CI"]
    fn test_delete_api_key() {
        if is_ci() {
            return;
        }

        // Clean up any existing key first
        let _ = delete_api_key();

        // Verify getting a non-existent key returns NotFound
        let result = get_api_key();
        assert!(matches!(result, Err(KeyringError::NotFound)));

        // Set and then delete
        set_api_key("temp-key").expect("should set API key");
        delete_api_key().expect("should delete API key");

        // Verify it's gone
        let result = get_api_key();
        assert!(matches!(result, Err(KeyringError::NotFound)));
    }

    #[test]
    fn test_error_display() {
        let not_found = KeyringError::NotFound;
        assert_eq!(format!("{}", not_found), "No API key found in keyring");

        let set_failed = KeyringError::SetFailed("disk full".to_string());
        assert_eq!(
            format!("{}", set_failed),
            "Failed to store API key: disk full"
        );

        let get_failed = KeyringError::GetFailed("access denied".to_string());
        assert_eq!(
            format!("{}", get_failed),
            "Failed to retrieve API key: access denied"
        );

        let delete_failed = KeyringError::DeleteFailed("locked".to_string());
        assert_eq!(
            format!("{}", delete_failed),
            "Failed to delete API key: locked"
        );

        let platform = KeyringError::PlatformNotSupported("no secretservice".to_string());
        assert_eq!(
            format!("{}", platform),
            "Keyring not supported on this platform: no secretservice"
        );
    }
}

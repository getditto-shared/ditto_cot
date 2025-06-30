use anyhow::Result;
use std::path::Path;

/// Loads environment variables from a .env file if it exists, otherwise uses existing environment variables.
/// This function is designed to be called at the beginning of test functions that require environment variables.
///
/// # Returns
/// `Result<()>` - Ok if the environment was successfully loaded or if no .env file was found.
///
/// # Example
/// ```rust
/// use anyhow::Result;
/// # mod test_utils {
/// #     pub fn load_test_env() -> Result<()> { Ok(()) }
/// # }
///
/// #[test]
/// fn my_test() -> Result<()> {
///     test_utils::load_test_env()?;
///     // Test code here can now use environment variables
///     Ok(())
/// }
/// ```
pub fn load_test_env() -> Result<()> {
    // Try to load from .env in the current directory
    let env_path = std::env::current_dir()?.join(".env");

    if env_path.exists() {
        // Load .env file if it exists
        dotenv::from_path(&env_path).map_err(|e| {
            eprintln!(
                "Warning: Failed to load .env file at {}: {}",
                env_path.display(),
                e
            );
            e
        })?;
    } else {
        // If no .env file, try parent directory (common in workspace setups)
        let parent_env_path = env_path
            .parent()
            .and_then(Path::parent)
            .map(|p| p.join(".env"));

        if let Some(parent_path) = parent_env_path {
            if parent_path.exists() {
                dotenv::from_path(&parent_path).map_err(|e| {
                    eprintln!(
                        "Warning: Failed to load .env file at {}: {}",
                        parent_path.display(),
                        e
                    );
                    e
                })?;
            }
        }
    }

    Ok(())
}

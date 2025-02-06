use std::path::PathBuf;

/// Returns the default config path `~/.kimainai.toml`
pub fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".kimainai.toml"))
        .expect("Could not determine home directory")
}

/// Resolves `~` to the user's home directory if present in the path
pub fn expand_tilde(s: &str) -> Result<PathBuf, String> {
    if s.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return Ok(home.join(&s[2..])); // Replace "~/" with home dir
        }
        return Err("Could not determine home directory".to_string());
    }
    Ok(PathBuf::from(s)) // Return original if no tilde found
}

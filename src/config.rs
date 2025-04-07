use serde::Deserialize;
use config;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub greeting_prefix: Option<String>,
    pub farewell_suffix: Option<String>,
}

/// Loads configuration from the specified file path.
/// Returns a Settings struct on success or a config::ConfigError on failure.
pub fn load_config(path: &str) -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?;
    settings.try_deserialize()
}

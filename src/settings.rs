use config::{Config, File, ConfigError};

#[derive(Debug, Deserialize)]
pub struct Settings {
    name: String
}

impl Settings {
    pub fn new(config_file: &str) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(config_file)).unwrap();
        return s.try_into();
    }
}
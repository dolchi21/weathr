use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        Self::load_from_path(&config_path)
    }

    pub fn load_from_path(path: &PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file at {:?}: {}", path, e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }

    fn get_config_path() -> Result<PathBuf, String> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else {
            dirs::config_dir().ok_or("Could not determine config directory")?
        };

        Ok(config_dir.join("weathr").join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_deserialize_valid() {
        let toml_content = r#"
[location]
latitude = 52.52
longitude = 13.41
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 52.52);
        assert_eq!(config.location.longitude, 13.41);
    }

    #[test]
    fn test_config_deserialize_negative_coordinates() {
        let toml_content = r#"
[location]
latitude = -33.8688
longitude = 151.2093
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, -33.8688);
        assert_eq!(config.location.longitude, 151.2093);
    }

    #[test]
    fn test_config_load_from_path_success() {
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_config.toml");

        let mut file = fs::File::create(&test_config_path).unwrap();
        writeln!(file, "[location]").unwrap();
        writeln!(file, "latitude = 40.7128").unwrap();
        writeln!(file, "longitude = -74.0060").unwrap();

        let config = Config::load_from_path(&test_config_path).unwrap();
        assert_eq!(config.location.latitude, 40.7128);
        assert_eq!(config.location.longitude, -74.0060);

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_config_load_from_path_file_not_found() {
        let nonexistent_path = PathBuf::from("/tmp/nonexistent_weathr_config_12345.toml");
        let result = Config::load_from_path(&nonexistent_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read config file"));
    }

    #[test]
    fn test_config_load_from_path_invalid_toml() {
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_invalid.toml");

        let mut file = fs::File::create(&test_config_path).unwrap();
        writeln!(file, "this is not valid toml {{{{").unwrap();

        let result = Config::load_from_path(&test_config_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse config"));

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_config_missing_latitude() {
        let toml_content = r#"
[location]
longitude = 13.41
"#;
        let result: Result<Config, _> = toml::from_str(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_missing_longitude() {
        let toml_content = r#"
[location]
latitude = 52.52
"#;
        let result: Result<Config, _> = toml::from_str(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_location_boundary_values() {
        let toml_content = r#"
[location]
latitude = 90.0
longitude = 180.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 90.0);
        assert_eq!(config.location.longitude, 180.0);
    }

    #[test]
    fn test_location_zero_coordinates() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 0.0);
        assert_eq!(config.location.longitude, 0.0);
    }
}

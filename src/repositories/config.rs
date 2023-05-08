use serde::Deserialize;
use std::fs;
use std::env;

#[derive(Deserialize, Debug, Clone)]
pub struct Source {
    pub id: u16,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sources: Vec<Source>,
}

impl Config {
    pub fn from_str(config: &str) -> Self {
        toml::from_str(&config).expect("Could not parse config file")
    }

    pub fn from_file(path: &str) -> Self {
        Self::from_str(&fs::read_to_string(path).expect("Unable to read file to string"))
    }

    pub fn from_env() -> Self {
        let config_path = env::var("CLIMBHUB_CONFIG").expect("CLIMBHUB_CONFIG not set");
        Self::from_file(&config_path)
    }

    pub fn get_source(&self, id: u16) -> Option<&Source> {
        self.sources.iter().find(|s| s.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_1() {
        let config = Config::from_str(
            r#"
            sources = [
                { id = 1, name = "Facebook", url = "https://www.facebook.com/" },
                { id = 2, name = "Google", url = "https://www.google.com/" },
            ]
            "#,
        );
        assert_eq!(config.sources.len(), 2);
        assert_eq!(config.sources[0].id, 1);
        assert_eq!(config.sources[0].name, "Facebook");
        assert_eq!(config.sources[0].url, "https://www.facebook.com/");
        assert_eq!(config.sources[1].id, 2);
        assert_eq!(config.sources[1].name, "Google");
        assert_eq!(config.sources[1].url, "https://www.google.com/");
    }

    #[test]
    fn test_from_str_2() {
        let config = Config::from_str(
            r#"
            [[sources]]
            id = 1
            name = "Rust Docs"
            url = "https://docs.rs/"

            [[sources]]
            id = 2
            name = "Twitter"
            url = "https://twitter.com/"
            "#,
        );
        assert_eq!(config.sources.len(), 2);
        assert_eq!(config.sources[0].id, 1);
        assert_eq!(config.sources[0].name, "Rust Docs");
        assert_eq!(config.sources[0].url, "https://docs.rs/");
        assert_eq!(config.sources[1].id, 2);
        assert_eq!(config.sources[1].name, "Twitter");
        assert_eq!(config.sources[1].url, "https://twitter.com/");
    }

    #[test]
    fn test_get_source() {
        let config = Config::from_str(
            r#"
            sources = [
                { id = 1, name = "Facebook", url = "https://www.facebook.com/" },
                { id = 2, name = "Google", url = "https://www.google.com/" },
            ]
            "#,
        );
        assert_eq!(config.get_source(1).unwrap().name, "Facebook");
        assert_eq!(config.get_source(2).unwrap().name, "Google");
        assert!(config.get_source(3).is_none());
    }
}
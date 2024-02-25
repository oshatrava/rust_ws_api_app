use config::{Config, File, FileFormat};

const PATH_TO_CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub port: usize,
    pub database_url: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let mut builder = Config::builder();
        builder = builder.add_source(File::new(PATH_TO_CONFIG_FILE, FileFormat::Toml));

        let config = match builder.build() {
            Ok(config) => config,
            Err(err) => panic!("Failed to build config: {}", err),
        };

        AppConfig {
            port: config.get_int("application.port").expect("Failed to get port") as usize,
            database_url: config.get_string("database.url").expect("Failed to get database url"),
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port as u16
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }
}
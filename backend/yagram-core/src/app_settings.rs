use actix_settings::{AtError, AtResult, BasicSettings, Settings};
use serde::Deserialize;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use tracing_subscriber::fmt::format;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OAuthSettings {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}

// If you edit this structure, you must sync the changes to app_settings.example.toml
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppSettings {
    pub postgres_url: String,
    pub redis_connection: String,
    pub oauth: OAuthSettings,
}

pub type YagramSettings = BasicSettings<AppSettings>;

pub struct SettingsInitializer();

impl SettingsInitializer {
    const APP_SETTINGS_TEMPLATE: &'static str = include_str!("./app_settings.example.toml");
    const CONFIG_PATH: &'static str = "./config.toml";
    fn write_toml_file() {
        if Settings::write_toml_file(Self::CONFIG_PATH).is_ok() {
            let file_path = Path::new(Self::CONFIG_PATH);
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            if let Err(e) = writeln!(file, "\n{}", Self::APP_SETTINGS_TEMPLATE) {
                eprintln!("{:?}", e);
                panic!("Failed to create clean config");
            }
        }
    }
    pub fn init() -> YagramSettings {
        Self::write_toml_file();
        YagramSettings::parse_toml(Self::CONFIG_PATH)
            .expect("Failed to parse `Settings` from 111config.toml")
    }
}

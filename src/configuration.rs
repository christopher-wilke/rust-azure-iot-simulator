use std::{fmt::Display, path::PathBuf};

use config::Config;
use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub iothub: IoTHub,
    pub device: Device,
}

#[derive(Debug, serde::Deserialize)]
pub struct IoTHub {
    pub hostname: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Device {
    pub device_id: String,
    pub shared_access_key: String,
}

#[derive(Debug)]
pub struct ConfigurationFileError;

impl Display for ConfigurationFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not read the configuration file properly")
    }
}

impl std::error::Error for ConfigurationFileError {}

#[derive(Debug)]
pub struct IoTHubConfig {}

pub trait ConfigurationFile {
    fn get_base_path() -> Result<PathBuf, ConfigurationFileError>;
    fn get_configuration_directory(
        base_path: PathBuf,
        sub_folder: String,
        file: String,
    ) -> Result<PathBuf, ConfigurationFileError>;
    fn get_deserialized_settings(config_file: PathBuf) -> Result<Settings, ConfigurationFileError>;
}

impl ConfigurationFile for IoTHubConfig {
    fn get_deserialized_settings(config_file: PathBuf) -> Result<Settings, ConfigurationFileError> {
        let cfg = Config::builder()
            .add_source(config::File::with_name(
                config_file.as_os_str().to_str().unwrap(),
            ))
            .build()
            .expect("Error while trying to create the Configuration Struct");

        cfg.try_deserialize::<Settings>()
            .report()
            .change_context(ConfigurationFileError)
            .attach_printable("Could not deserialze the configuration struct")
    }

    fn get_base_path() -> Result<PathBuf, ConfigurationFileError> {
        std::env::current_dir()
            .report()
            .change_context(ConfigurationFileError)
            .attach_printable("Could not read the base path")
    }

    fn get_configuration_directory(
        base_path: PathBuf,
        sub_folder: String,
        file: String,
    ) -> Result<PathBuf, ConfigurationFileError> {
        let path_buf = base_path.join(sub_folder.clone()).join(file.clone());

        if path_buf.exists() {
            Ok(path_buf)
        } else {
            return Err(
                Report::new(ConfigurationFileError)
                .attach_printable(
                    format!("./{sub_folder}/{file} not available. Make sure to create the file and insert the values.")
                )
            );
        }
    }
}

impl IoTHubConfig {
    pub fn get_settings() -> Result<Settings, ConfigurationFileError> {
        let base_path = IoTHubConfig::get_base_path()?;
        let cfg_file = IoTHubConfig::get_configuration_directory(
            base_path,
            "configuration".to_string(),
            "base.yaml".to_string(),
        )?;
        let settings = IoTHubConfig::get_deserialized_settings(cfg_file)?;
        Ok(settings)
    }
}

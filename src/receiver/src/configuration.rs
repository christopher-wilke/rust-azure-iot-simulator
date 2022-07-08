use std::{fmt::Display, path::PathBuf};

use config::{Config, ConfigError};
use error_stack::{Context, IntoReport, Result, ResultExt, Report};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub iothub: IoTHub,
    pub device: Device,
}

#[derive(serde::Deserialize)]
pub struct IoTHub {
    pub hostname: String,
}

#[derive(serde::Deserialize)]
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
pub struct IoTHubConfig {
    // base_path: PathBuf
}

pub trait ConfigurationFile {
    fn get_base_path() -> Result<PathBuf, ConfigurationFileError>;
    fn get_configuration_directory(
        base_path: PathBuf,
        sub_folder: String, 
        file: String
    ) -> Result<PathBuf, ConfigurationFileError>;
}

impl ConfigurationFile for IoTHubConfig {
    fn get_base_path() -> Result<PathBuf, ConfigurationFileError> {
        std::env::current_dir()
            .report()
            .change_context(ConfigurationFileError)
            .attach_printable(format!("Could not read the base path"))
    }

    fn get_configuration_directory(
        base_path: PathBuf,
        sub_folder: String, 
        file: String
    ) -> Result<PathBuf, ConfigurationFileError> {
        let path_buf = base_path
            .join(sub_folder.clone())
            .join(file.clone());

        if path_buf.exists() {
            Ok(path_buf)
        } else {
            return Err(
                Report::new(ConfigurationFileError)
                .attach_printable(format!("./{sub_folder}/{file} not valid."))
            )
        }
    }
}

impl IoTHubConfig {

    pub fn new() -> Result<Self, ConfigurationFileError> {
        let base_path = IoTHubConfig::get_base_path()?;
        let cfg_dir = IoTHubConfig::get_configuration_directory(
            base_path, 
            "configuration".to_string(), 
            "base.yaml".to_string()
        )?;

        println!("{cfg_dir:?}");

        Ok(IoTHubConfig { 
            
        })
    }
}

pub fn get_settings() -> Result<String, ConfigurationFileError> {
    let iot_hub_cfg = IoTHubConfig::new()?;
    
    println!("{iot_hub_cfg:?}");

    Ok("hello world".to_string())
}
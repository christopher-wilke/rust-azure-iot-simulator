use config::{Config, ConfigError};

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

pub fn get_deserialized_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("could not get the current directory");
    let config_dir = base_path.join("configuration").join("base.yaml");

    let config_file = config_dir
        .as_os_str()
        .to_str()
        .expect("could not parse string");

    let cfg = Config::builder()
        .add_source(config::File::with_name(config_file))
        .build()
        .expect("Could not parse the configuration file");

    cfg.try_deserialize::<Settings>()
}

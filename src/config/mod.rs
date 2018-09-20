pub mod keys;
pub mod options;
use error::*;

use std::fs as stdfs;
use std::path::PathBuf;
use dirs;
use toml;


#[derive(Serialize,Deserialize)]
struct Config {
    #[serde(default)]
    KeyMaps: keys::KeyMaps,
    #[serde(default)]
    Options: options::ConfigOptions,
}

impl Config {
    pub fn load() -> Result<Self>{
        let data_path: PathBuf = dirs::config_dir().ok_or(ErrorKind::DirNotFound {
            dirname: String::from("CONFIG_DIR"),
        })?;
        let data_path = data_path.join("marcos");
        if !data_path.exists() {
            stdfs::create_dir_all(&data_path).expect("Cannot create data_dir");
        }
        let config_file = data_path.join("config.toml");
        debug!("Loading theme from file: {:?}", config_file);
        if !config_file.is_file() {
            stdfs::File::create(&config_file).expect("Failed to create asset file");
        }
        let config_str = stdfs::read_to_string(config_file)?;
        let config_data: Config = toml::from_str(config_str.as_str())?;
        Ok(config_data)
    }
}

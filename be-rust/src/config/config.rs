use config::{Config, File, FileFormat};

use crate::{errors::AppResult, model::config_model::ConfigModel};

pub fn load_config() -> AppResult<ConfigModel> {
    let config_path = if std::path::Path::new("config/config.yaml").exists() {
        "config/config.yaml"
    } else if std::path::Path::new("config/config.example.yaml").exists() {
        "config/config.example.yaml"
    } else {
        "config/config.yaml"
    };

    let conf: ConfigModel = Config::builder()
        .add_source(File::new(config_path, FileFormat::Yaml))
        .build()?
        .try_deserialize()?;

    Ok(conf)
}

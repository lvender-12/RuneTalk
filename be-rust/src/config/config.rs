use config::{Config, File, FileFormat};

use crate::{errors::AppResult, model::config_model::ConfigModel};

pub fn load_config() -> AppResult<ConfigModel> {
    let conf: ConfigModel = Config::builder()
        .add_source(File::new("config/config.yaml", FileFormat::Yaml))
        .build()?
        .try_deserialize()?;

    Ok(conf)
}

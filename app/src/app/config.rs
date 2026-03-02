use super::APP_ID;
use super::APP_NAME;
use super::TreeViewConfig;

use confy::ConfigStrategy;
use confy::change_config_strategy;
use confy::load;
use confy::store;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct AppConfig {
    tv_config: TreeViewConfig,
}

impl From<AppConfig> for TreeViewConfig {
    fn from(ac: AppConfig) -> Self {
        ac.tv_config
    }
}

impl From<TreeViewConfig> for AppConfig {
    fn from(tv_config: TreeViewConfig) -> Self {
        AppConfig { tv_config }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        load_config()
    }

    pub fn store(self) {
        store_config(self);
    }
}

#[cfg(debug_assertions)]
fn print_config_file_path() {
    let p = confy::get_configuration_file_path(APP_ID, Some(APP_NAME));
    println!(
        "{APP_NAME} ({APP_ID}) configuration file path: {}",
        match p {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(err) => err.to_string(),
        }
    );
}

fn store_config(config: AppConfig) {
    change_config_strategy(ConfigStrategy::Native);
    #[cfg(debug_assertions)]
    print_config_file_path();
    match store(APP_ID, Some(APP_NAME), config) {
        Ok(_) => {}
        Err(err) => {
            // ToDo: Handle error?
            dbg!(err);
        }
    };
}

fn load_config() -> AppConfig {
    change_config_strategy(ConfigStrategy::Native);
    #[cfg(debug_assertions)]
    print_config_file_path();
    match load(APP_ID, Some(APP_NAME)) {
        Ok(v) => v,
        Err(err) => {
            // ToDo: Handle error?
            dbg!(err);
            let config = AppConfig::default();
            config.store();
            config
        }
    }
}

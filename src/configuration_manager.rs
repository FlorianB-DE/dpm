use std::collections::HashMap;

use confy::load;
use serde_derive::{Deserialize, Serialize};

use crate::Errors;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub installed_programs: HashMap<String, Vec<String>>,
}

impl Drop for AppConfig {
    fn drop(&mut self) {
        let result = confy::store("dpm", "settings", self);
        if result.is_err() {
            // can not panic
            eprintln!("Could not save config file: {}", result.unwrap_err());
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: str!(env!("CARGO_PKG_VERSION")),
            installed_programs: HashMap::new(),
        }
    }
}

/// ATTENTION! 
/// 
/// Having multiple calls to this function may lead to data loss due to the 
/// config getting saved to file once it's droped.
pub fn get_config() -> Result<AppConfig, Errors> {
    load::<AppConfig>("dpm", "settings").or_else(|e| {
        eprintln!("Could not load config.\n{e}");
        Err(Errors::ConfigLoadFailed)
    })
}

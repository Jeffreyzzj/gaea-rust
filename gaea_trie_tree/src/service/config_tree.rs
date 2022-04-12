use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigTree {
    pub path: String,
}

impl ConfigTree {
    pub fn new(file_path: String) -> ConfigTree {
        ConfigTree {
            path: file_path,
        }
    }
    pub fn init_config_tree(&self) {
        println!("aaa bbb ccc hahaha");
    }
}
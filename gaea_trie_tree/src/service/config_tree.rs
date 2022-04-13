use std::sync::RwLock;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref CONFIG_TREE: RwLock<ConfigTree> = RwLock::new(ConfigTree::new("".to_string()));
}

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

    // 获得tree_conf信息
    pub fn get_tree_conf() {
        let config_info = CONFIG_TREE.read().unwrap().to_owned();
        println!("config: \n{:?}", config_info);
    }
    
    // 重写配置路径
    pub fn re_write_config_path(path: String) {
        let mut new_config_tree = CONFIG_TREE.write().unwrap();
        *new_config_tree = ConfigTree::new(path);
    }
}
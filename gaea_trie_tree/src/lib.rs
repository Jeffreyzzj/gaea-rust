#[macro_use(lazy_static)]
extern crate lazy_static;
use std::sync::RwLock;
use std::collections::HashMap;

mod service;
use service::tree::trie_tree::{GaeaTrieTree};
use service::config_tree::{ConfigTree};
use service::file_uitl::{FileUtil};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

lazy_static! {
    static ref CONFIG_TREE: RwLock<ConfigTree> = RwLock::new(ConfigTree::new("".to_string()));

    static ref ID_CHAR_MAP: HashMap<i32, String> = FileUtil::init_id_char_map();

    static ref CHAR_ID_MAP: HashMap<String, i32> = FileUtil::init_char_id_map();
}

pub fn init_config(path: String) {
    // 重写路径
    re_write_config_path(path);
    // 读取数据
    get_tree_conf();
}

pub fn get_tree_conf() {
    let config_info = CONFIG_TREE.read().unwrap().to_owned();
    println!("config: \n{:?}", config_info);
}

pub fn re_write_config_path(path: String) {
    let mut new_config_tree = CONFIG_TREE.write().unwrap();
    *new_config_tree = ConfigTree::new(path);
}
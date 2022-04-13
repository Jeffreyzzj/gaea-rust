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
    static ref config_tree: RwLock<ConfigTree> = RwLock::new(ConfigTree::new(test_get_str()));

    static ref id_char_map: HashMap<i32, String> = FileUtil::init_id_char_map();

    static ref char_id_map: HashMap<String, i32> = FileUtil::init_char_id_map();
}

pub fn test_get_str() -> String {   
    String::from("abcccc")
}

pub fn init_config(path: String) {
    
    write_str(path);
    //read_conf();
}

pub fn read_conf() {
    let aa = config_tree.read().unwrap().to_owned();
    println!("hahaha {:?}", aa);
}

pub fn write_str(path: String) {
    let mut new_config_tree = config_tree.write().unwrap();
    *new_config_tree = ConfigTree::new(path);
}
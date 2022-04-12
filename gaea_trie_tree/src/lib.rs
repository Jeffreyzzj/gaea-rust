#[macro_use(lazy_static)]
extern crate lazy_static;
use std::sync::RwLock;

mod service;
use service::tree::trie_tree::{GaeaTrieTree};
use service::config_tree::{ConfigTree};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// pub fn CreateTrieTree() -> GaeaTrieTree {
//     let trie_tree_p = GaeaTrieTree::new();
//     return trie_tree_p;
// }

lazy_static! {
    //static ref config_tree: ConfigTree = ConfigTree::new(test_get_str());
    static ref config_tree: RwLock<ConfigTree> = RwLock::new(ConfigTree::new(test_get_str()));
}

pub fn test_get_str() -> String {   
    String::from("abcccc")
}

pub fn init_config(path: String) {
    
    write_str(path);
    read_conf();
}

pub fn read_conf() {
    let aa = config_tree.read().unwrap().to_owned();
    println!("hahaha {:?}", aa);
}

pub fn write_str(path: String) {
    let mut new_config_tree = config_tree.write().unwrap();
    *new_config_tree = ConfigTree::new(path);
}
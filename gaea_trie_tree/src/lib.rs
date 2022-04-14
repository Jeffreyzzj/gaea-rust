#[macro_use(lazy_static)]
extern crate lazy_static;
use std::collections::HashMap;

mod service;
use service::tree::trie_tree::{GaeaTrieTree};
use service::config_tree::{ConfigTree};
use service::character::character_util::{CharacterUtil};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub fn init_config(path: String) {
    // 重写路径
    //ConfigTree::re_write_config_path(path);
    // 读取数据
    //ConfigTree::get_tree_conf();
    CharacterUtil::get_id_char_map(&1);
}

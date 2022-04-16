#[macro_use(lazy_static)]
extern crate lazy_static;
use std::collections::HashMap;

mod service;
use service::tree::trie_tree::{GaeaTrieTree};
//use service::config_tree::{ConfigTree};
//use service::character::character_util::{CharacterUtil};
//use service::character::character_util::{CharacterUtil};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub fn init_config(path: String) {
    //CharacterUtil::query_to_id_list("你好呀, rust".to_string());
    GaeaTrieTree::insert_by_query("你好呀, rust".to_string());
    GaeaTrieTree::get_tree_info();
    // let ch_str = String::from("吃");
    // let res = CharacterUtil::get_char_id_map(&ch_str);
    // match res {
    //     Some(c) => println!("value {}", c),
    //     _ => println!("is null")
    // }
}

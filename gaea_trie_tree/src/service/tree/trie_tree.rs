use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::service::character::character_util::{CharacterUtil};

lazy_static! {
    static ref TRIE_TREE_P: RwLock<GaeaTrieTree> = RwLock::new(GaeaTrieTree::new());
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaeaTrieTree {
    pub is_word: bool,
    pub next_id_map: HashMap<i32, GaeaTrieTree>,
}

impl GaeaTrieTree {
    
    pub fn new() -> GaeaTrieTree {
        GaeaTrieTree{
            is_word: false,
            next_id_map: HashMap::new(),
        }
    }

    pub fn insert_by_query(query: String) {
        if query.len() == 0 {
            return;
        }
        let id_list: Vec<i32> = CharacterUtil::query_to_id_list(query);
        //print!("id_list value : {:?}", id_list);

        let mut new_trie_tree = &mut TRIE_TREE_P.write().unwrap();
        //*new_config_tree = TRIE_TREE_P::new(path);
        //*new_config_tree = GaeaTrieTree::new();
        //let root = *new_trie_tree;
        for id in id_list {
            // new_trie_tree.next_id_map.entry(id).or_insert(GaeaTrieTree{
            //     is_word: false,
            //     next_id_map: HashMap::new(),
            // });
            let mid_map = &mut new_trie_tree.next_id_map;
            if !mid_map.contains_key(&id) {
                mid_map.insert(id, GaeaTrieTree::new());
            }

            new_trie_tree = match mid_map.get(&id) {
                Some(tree) => RwLockWriteGuard::new(tree),
                None => return
            };
        }

        new_trie_tree.is_word = true;
    }

    // 获得tree_conf信息
    pub fn get_tree_info() {
        let config_info = TRIE_TREE_P.read().unwrap().to_owned();
        println!("config: \n{:?}", config_info);
    }
}


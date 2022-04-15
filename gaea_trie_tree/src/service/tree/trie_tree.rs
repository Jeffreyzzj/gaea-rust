use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref CONFIG_TREE: RwLock<GaeaTrieTree> = RwLock::new(GaeaTrieTree::new());
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
}


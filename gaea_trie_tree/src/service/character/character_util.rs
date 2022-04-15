use std::collections::HashMap;

//mod service;
//use service::file_util::{FileUtil};
use crate::service::file_util::{FileUtil};

lazy_static! {
    static ref ID_CHAR_MAP: HashMap<i32, String> = FileUtil::init_id_char_map();
    static ref CHAR_ID_MAP: HashMap<String, i32> = FileUtil::init_char_id_map();
}

pub struct CharacterUtil {
}

impl CharacterUtil {
    pub fn get_id_char_map(id: &i32) -> Option<&String> {
        return ID_CHAR_MAP.get(id);
    }

    pub fn get_char_id_map(ch: &String) -> Option<&i32> {
        return CHAR_ID_MAP.get(ch);
    }

    // 将query转化为ID列表
    pub fn query_to_id_list(query: String) -> Vec<i32> {
        let mut id_list: Vec<i32> = Vec::new();

        for c in query.chars() {
            let c_str = c.to_string();
            let id_infos = CharacterUtil::get_char_id_map(&c_str);
            if let Some(id) = id_infos {
                //let v = *id;
                id_list.push(*id);
            }   
        }
        print!("id_list value : {:?}", id_list);

        return id_list;
    }
}
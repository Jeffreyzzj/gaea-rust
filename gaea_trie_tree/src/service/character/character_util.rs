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
}
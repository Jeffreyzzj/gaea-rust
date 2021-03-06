use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

use log::{info};

pub struct FileUtil {}

impl FileUtil {
    // 初始化 ID - char map
    pub fn init_id_char_map() -> HashMap<i32, String> {
        let id_char_res = FileUtil::read_id_char_file();
        let id_char_map: HashMap<i32, String> = match id_char_res {
            Ok(id_char_map_res) => {
                id_char_map_res
            }, 
            Err(e) => {
                panic!("Error");
                HashMap::new()
            }
        };
        return id_char_map;
    }

    pub fn read_id_char_file() -> Result<HashMap<i32, String>> {
        //let file = File::open("/home/work/code/gaea-rust/gaea_trie_tree/resources/id_char.txt")?;
        let file = File::open("/Users/zengzijie/zzj/code/rust_code/gaea-rust/gaea_trie_tree/resources/id_char.txt")?;
        let mut id_char_map: HashMap<i32, String> = HashMap::new();
        for line in BufReader::new(file).lines() {

            let info_str = match line {
                Ok(info_str) => {
                    info_str
                },
                Err(err) => {
                    println!("{}", err);
                    continue
                }
            };

            let line_infos: Vec<&str> = info_str.split(":").collect();

            let id = line_infos[0].parse::<i32>().unwrap();
            id_char_map.insert(id, line_infos[1].to_string());
        }
        Ok(id_char_map)
    }

    // 初始化 char - ID map
    pub fn init_char_id_map() -> HashMap<String, i32> {
        let char_id_res = FileUtil::read_char_id_file();
        let char_id_map: HashMap<String, i32> = match char_id_res {
            Ok(char_id_map_res) => {
                char_id_map_res
            }, 
            Err(e) => {
                panic!("Error: {}", e);
                HashMap::new()
            }
        };
        return char_id_map;
    }

    pub fn read_char_id_file() -> Result<HashMap<String, i32>> {
        //let file = File::open("/home/work/code/gaea-rust/gaea_trie_tree/resources/char_id.txt")?;
        let file = File::open("/Users/zengzijie/zzj/code/rust_code/gaea-rust/gaea_trie_tree/resources/id_char.txt")?;
        let mut char_id_map: HashMap<String, i32> = HashMap::new();
        for line in BufReader::new(file).lines() {

            let info_str = match line {
                Ok(info_str) => {
                    info_str
                },
                Err(err) => {
                    println!("{}", err);
                    continue
                }
            };

            let line_infos: Vec<&str> = info_str.split(":").collect();

            let id = line_infos[1].parse::<i32>().unwrap();
            char_id_map.insert(line_infos[0].to_string(), id);
        }
        Ok(char_id_map)
    }

}
use std::{env, fs};
use discord::model::*;
use std::path::PathBuf;
use std::collections::HashMap;


pub fn load(mut memes: HashMap<String, PathBuf>) -> HashMap<String, PathBuf> {
    for entry in fs::read_dir("./memes").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ptk = path.clone();
        for meme in fs::read_dir(path).unwrap() {
            let meme = meme.unwrap();
            let memepath = meme.path();
            if let Some(message) = ptk.file_name().unwrap().to_str() {
                println!("keyword {:?}, meme url {:?}", message, meme);
                memes.insert(String::from(message), memepath.clone());
            }
        }
    }

    memes.clone()
}

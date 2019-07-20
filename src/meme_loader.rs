use std::{env, fs};
use discord::model::*;
use std::path::PathBuf;
use std::collections::HashMap;


pub fn load(mut memes: HashMap<String, Vec<PathBuf>>) -> HashMap<String, Vec<PathBuf>> {
    for entry in fs::read_dir("./memes").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ptk = path.clone();
        for meme in fs::read_dir(path).unwrap() {
            let meme = meme.unwrap();
            let memepath = meme.path();
            if let Some(message) = ptk.file_name().unwrap().to_str() {
                println!("keyword {:?}, meme url {:?}", message, meme);
                let mut kl = memes.clone();
                let mut empty = Vec::<PathBuf>::new();
                let meme_list = kl.get_mut(String::from(message).as_str()).unwrap_or(&mut empty);
                meme_list.push(memepath.clone());
                memes.insert(String::from(message), meme_list.clone());
            }
        }
    }

    memes.clone()
}

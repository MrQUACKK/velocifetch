use std::{env, fs};

pub struct Info {
    pub name: String, // The name of what the info is getting (ex: host, user, etc.)
    pub data: String, // The actual info
    // TODO: Add cachable and cache
    // cachable: bool, // If this info is cachable
    // cache: bool, // Wether or not to cache the info; requires cachable to be true
}
impl Info {
    pub fn print(&self){
        println!("{} -> {}", self.name, self.data.trim());
    }
    pub fn cache(&self){
        let cache_file: &str = "/home/abdullah/.cache/velocifetch";
        fs::write(cache_file, format!("{}{}{}", self.name, "->", self.data))
            .expect("~/.cache/velocifetch was not found");
    }
}

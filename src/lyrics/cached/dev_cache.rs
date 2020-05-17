use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;

use serde_json;

use crate::utils::stringify_map_keys;
use super::*;

type HashMapCache = HashMap<SongDescriptor, String>;

static mut CACHE: Option<Rc<RefCell<HashMapCache>>> = None;

pub struct DevCache {
    cache: Rc<RefCell<HashMap<SongDescriptor, String>>>,
}

impl DevCache {
    pub fn new() -> Self {
        let cache = unsafe {
            match &CACHE {
                Some(cache) => cache.clone(),
                None => {
                    let new_cache = Self::make_cache("./cache/lyrics-cache.json");

                    CACHE = Some(new_cache.clone());
                    new_cache
                }
            }
        };

        DevCache { cache }
    }

    fn make_cache<'a>(path: &'a str) -> Rc<RefCell<HashMapCache>> {
        File::open(path)
            .map(|mut file| {
                let mut serialized = String::new();
                file.read_to_string(&mut serialized).unwrap();

                let cache: HashMap<SongDescriptor, String> = serde_json::from_str(&serialized)
                    .unwrap();

                Rc::new(RefCell::new(cache))
            })
            .unwrap()
    }
}

impl Cache for DevCache {
    fn save(&mut self, song: &SongDescriptor, lyrics: String) -> Result<(), String> {
        self.cache.borrow_mut().insert(song.clone(), lyrics);

        Ok(())
    }

    fn load(&self, song: &SongDescriptor) -> Result<Option<String>, String> {
        match self.cache.borrow().get(song) {
            Some(lyrics) => Ok(Some(String::from(lyrics))),
            None => Ok(None)
        }
    }

    fn write_back(&mut self) -> Result<(), String> {
        // Swap the cache out just long enough to write it to disk and swap it back.
        // We're fulfilling our childhoold dreams of becoming a magician!
        let cache = self.cache.replace(HashMap::new());

        let write_back_result = serde_json::to_string(&stringify_map_keys(&cache))
            .map(|serialized_cache| {
                File::open("./cache/lyrics-cache.json")
                    .map(|mut file| {
                        println!("writing cache to disk...");

                        file.write_all(serialized_cache.as_bytes())
                            .map(|_| println!("cahe written to disk!"))
                            .unwrap()
                    })
                    .unwrap()
            });

        // Replace the cache before checking if the write-back operation succeeded so
        // if we do end up panicking it will be _after_ we've restored everything.
        self.cache.replace(cache);

        match &write_back_result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    #[ignore]
    // Used to populate test cache -- you probably don't need or want to run this.
    fn populate_cache() {
        let mut cache = HashMap::new();
        cache.insert(serde_json::to_string(&SongDescriptor { name: "foo".to_string(), artist: "bar".to_string(), uri: None }).unwrap(), "foo bar baz".to_string());

        let serialized_cache = serde_json::to_string(&cache).unwrap();

        File::create("./test_data/cached/test_cache_temp.json")
            .unwrap()
            .write_all(serialized_cache.as_bytes())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn can_load_cache() {
        let cache = DevCache::make_cache("../../../test_data/cached/test_cache_temp.json");

        let key = SongDescriptor{ name: "foo".to_string(), artist: "bar".to_string(), uri: None };
        let value = "foo bar baz".to_string();

        assert_eq!(cache.borrow().get(&key).unwrap(), &value);
    }
}
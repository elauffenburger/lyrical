use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::rc::Rc;

use serde_json;

use crate::utils::stringify_map_keys;
use super::*;

type HashMapCache = HashMap<SongDescriptor, CacheEntry>;

static mut CACHE: Option<Rc<RefCell<HashMapCache>>> = None;

const CACHE_LOCATION: &'static str = "./cache/lyrics.json";

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct DevCacheOptions {
    write_eagerly: bool,
}

pub struct DevCache {
    cache: Rc<RefCell<HashMapCache>>,
    options: DevCacheOptions,
}

impl DevCache {
    pub fn new(options: DevCacheOptions) -> Self {
        let cache = unsafe {
            match &CACHE {
                Some(cache) => cache.clone(),
                None => {
                    let new_cache = Self::make_cache(CACHE_LOCATION);

                    CACHE = Some(new_cache.clone());
                    new_cache
                }
            }
        };

        DevCache { cache, options }
    }

    fn make_cache<'a>(path: &'a str) -> Rc<RefCell<HashMapCache>> {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map(|mut file| {
                let mut serialized = String::new();
                file.read_to_string(&mut serialized).unwrap();

                let cache = serde_json::from_str::<HashMap<String, CacheEntry>>(&serialized)
                    .map(|cache_with_serialized_keys| 
                        cache_with_serialized_keys
                            .into_iter()
                            .fold(HashMap::new(), |mut acc, (key, value)| {
                                acc.insert(serde_json::from_str(&key).unwrap(), value);

                                acc
                            })
                    )
                    .unwrap();

                Rc::new(RefCell::new(cache))
            })
            .unwrap()
    }
}

impl Cache for DevCache {
    fn save(&mut self, song: &SongDescriptor, entry: CacheEntry) -> Result<(), String> {
        self.cache.borrow_mut().insert(song.clone(), entry);

        if self.options.write_eagerly {
            self.write_back()?;
        }

        Ok(())
    }

    fn load(&self, song: &SongDescriptor) -> Result<Option<CacheEntry>, String> {
        match self.cache.borrow().get(song) {
            Some(result) => Ok(Some(result.clone())),
            None => Ok(None)
        }
    }

    fn write_back(&mut self) -> Result<(), String> {
        // Swap the cache out just long enough to write it to disk and swap it back.
        // We're fulfilling our childhoold dreams of becoming a magician!
        let cache = self.cache.replace(HashMap::new());

        let write_back_result = serde_json::to_string(&stringify_map_keys(&cache))
            .map(|serialized_cache| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(CACHE_LOCATION)
                    .map(|mut file| {
                        println!("writing cache to disk...");

                        file.write_all(serialized_cache.as_bytes())
                            .map(|_| println!("cache written to disk!"))
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

impl Debug for DevCache {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str("DevCache { }")
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
        cache.insert(serde_json::to_string(&SongDescriptor { name: "foo".to_string(), artist: "bar".to_string(), uri: None }).unwrap(), CacheEntry::Success("foo bar baz".to_string()));

        let serialized_cache = serde_json::to_string(&cache).unwrap();

        File::create("./test_data/cached/test_cache_temp.json")
            .unwrap()
            .write_all(serialized_cache.as_bytes())
            .unwrap();
    }

    #[test]
    fn can_load_cache() {
        let cache = DevCache::make_cache("./test_data/cached/test_cache_temp.json");

        let key = SongDescriptor{ name: "foo".to_string(), artist: "bar".to_string(), uri: None };
        let value = CacheEntry::Success("foo bar baz".to_string());

        assert_eq!(cache.borrow().get(&key).unwrap(), &value);
    }
}
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use serde_json;

use super::*;

macro_rules! make_cache {
    ($input:expr) => {{
        let serialized = include_str!($input);
        
        let cache: HashMap<SongDescriptor, String> = serde_json::from_str(serialized)
            .unwrap();

        Rc::new(RefCell::new(cache))
    }};
}

static mut CACHE: Option<Rc<RefCell<HashMap<SongDescriptor, String>>>> = None;

struct CachedLyricsFetcher<'a> {
    cache: Rc<RefCell<HashMap<SongDescriptor, String>>>,
    fallback: &'a dyn LyricsFetcher,
}

impl<'a> CachedLyricsFetcher<'a> {
    pub fn new(fallback: &'a dyn LyricsFetcher) -> Self {
        let instance_cache = unsafe {
            match &CACHE {
                Some(cache) => cache.clone(),
                None => {
                    let new_cache = make_cache!("../../cached/lyrics-cache.json");

                    CACHE = Some(new_cache.clone());
                    new_cache
                }
            }
        };
        
        CachedLyricsFetcher { cache: instance_cache, fallback }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;

    use maplit::hashmap;

    use super::*;

    #[test]
    #[ignore]
    // Used to populate test cache -- you probably don't need or want to run this.
    fn populate_cache() {
        let mut cache = HashMap::new();
        cache.insert(serde_json::to_string(&SongDescriptor { name: "foo".to_string(), artist: "bar".to_string(), uri: None }).unwrap(), "foo bar baz".to_string());

        let serialized_cache = serde_json::to_string(&cache).unwrap();

        File::create("test_data/cached/test_cache_temp.json")
            .unwrap()
            .write_all(serialized_cache.as_bytes())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn can_load_cache() {
        let cache = make_cache!("../../test_data/cached/test_cache_temp.json");

        let key = SongDescriptor{ name: "foo".to_string(), artist: "bar".to_string(), uri: None };
        let value = "foo bar baz".to_string();

        assert_eq!(cache.borrow().get(&key).unwrap(), &value);
    }
}
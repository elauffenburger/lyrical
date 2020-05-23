use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SongUri {
    MusixMatchUri(String)
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SongDescriptor {
    pub name: String,
    pub artist: String,
    pub uri: Option<SongUri>,
}

impl ToString for SongDescriptor {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
use std::str;

use crate::storages;

pub struct AssetService {}

impl AssetService {
    pub fn get_text_file(path: &str) -> Result<String, ()> {
        let embed = match storages::local::asset::Asset::get(path) {
            Some(embed) => embed,
            None => return Err(()),
        };

        let buff = embed.data.into_owned();
        match str::from_utf8(&buff) {
            Ok(data) => Ok(data.to_owned()),
            Err(_) => Err(()),
        }
    }

    pub fn get_raw(path: &str) -> Result<Vec<u8>, ()> {
        match storages::local::asset::Asset::get(path) {
            Some(embed) => Ok(embed.data.into_owned()),
            None => Err(()),
        }
    }
}

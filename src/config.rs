use std::{fs::File, io::{Error, Read}, path::Path};

use crate::api::Status;
use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize)]
pub struct Config{
    pub token: String,
    pub duration: u64,
    pub status_list: Vec<Status>,
    pub retry_time: u64
}

impl Config {
    pub fn init(path: &Path) -> Result<Config, Error>{
        let data = Config::read_file(path)?;

        let config:Config = serde_json::from_str(&data)?;

        Ok(config)
    }

    fn read_file(path: &Path) -> Result<String, Error>{
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}
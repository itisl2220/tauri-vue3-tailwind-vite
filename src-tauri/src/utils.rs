extern crate scraper;
use scraper::{Html, Selector};
use serde::Deserialize;
use sled::Db;
use tauri::regex::Regex;
use crate::error::Result;

// 解析正则
pub fn extract_regex(text: &str, regex: &str) -> String {
    let re = Regex::new(regex).unwrap();
    let caps = re.captures(text).unwrap();
    caps[1].to_string()
}

// 解析html
pub fn extract_html(text: &str, selector: &str) -> Result<Vec<String>> {
    let fragment = Html::parse_fragment(text);
    let selector = Selector::parse(selector)?;
    let result: Vec<_> = fragment
            .select(&selector)
            .rev()
            .map(|e| e.inner_html())
            .collect();
    Ok(result)
}



#[derive(Debug, Clone)]
pub struct SledDb {
    db: Db,
}

impl Default for SledDb {
    fn default() -> Self {
        Self::new()
    }
}

impl SledDb {
    pub fn new() -> Self {
        let db = sled::open(".store").expect("open local_storage");
        Self { db: db.clone() }
    }

    pub fn set<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let value = serde_json::to_vec(value)?;
        self.db.insert(key, value.to_owned())?;
        Ok(())
    }

    pub fn get<T: for<'a> Deserialize<'a>>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        match self.db.get(key).unwrap_or(None) {
            Some(value) => Ok(Some(serde_json::from_slice::<T>(&value)?)),
            None => Ok(None),
        }
    }
}




use crate::Error;
use randomizer::Randomizer;
use std::collections::HashMap;

pub(crate) struct UrlStore {
    store: HashMap<String, String>,         // url -> short_url
    reverse_store: HashMap<String, String>, // short_url -> url
}

impl UrlStore {
    pub(crate) fn new() -> Self {
        UrlStore {
            store: HashMap::new(),
            reverse_store: HashMap::new(),
        }
    }

    pub(crate) fn get_url(&mut self, short_url: &str) -> Option<String> {
        self.reverse_store.get(short_url).cloned()
    }

    pub(crate) fn store_url(&mut self, url: &str) -> Result<String, Error> {
        if let Some(short_url) = self.store.get(url) {
            return Ok(short_url.clone());
        }

        let short_url = shorten_url(&url)?;
        self.store.insert(url.to_owned(), short_url.clone());
        self.reverse_store.insert(short_url.clone(), url.to_owned());

        Ok(short_url)
    }
}

fn shorten_url(_url: &str) -> Result<String, Error> {
    let short_url = Randomizer::ALPHABETICAL(16)
        .string()
        .map_err(|e| Error::RandomStringGenerate(e))?;
    Ok(short_url)
}

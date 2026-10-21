use arc_swap::ArcSwap;
use std::{
    collections::BTreeMap,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};

use crate::{error::Error, helpers};

const TTL: Duration = Duration::from_secs(3600);

static LAST_FETCHED: LazyLock<ArcSwap<SystemTime>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(SystemTime::now())));
static CONTENT: LazyLock<ArcSwap<BTreeMap<(u64, u8), String>>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(match fetch_manuf() {
        Ok(content) => {
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
            content
        }
        Err(e) => {
            eprintln!("{e} - fallback to offline index");
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
            helpers::parse_content(include_str!("manuf.txt"))
        }
    })));

#[cfg_attr(docsrs, doc(cfg(feature = "online")))]
pub fn lookup(mac: impl Into<String>) -> Result<Option<String>, Error> {
    // Update the content if TTL has passed
    let last_fetched = LAST_FETCHED.load();

    if SystemTime::now() > *last_fetched.as_ref() + TTL {
        if let Ok(content) = fetch_manuf() {
            CONTENT.store(Arc::new(content));
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
        }
    }

    let content = CONTENT.load();

    crate::lookup_from_content(&content, mac)
}

fn fetch_manuf() -> Result<BTreeMap<(u64, u8), String>, Error> {
    let response = reqwest::blocking::get(
        "https://raw.githubusercontent.com/kkrypt0nn/manuf/refs/heads/main/manuf.txt",
    ).map_err(Error::SendManufRequest)?.error_for_status().map_err(Error::ManufResponseStatusCode)?;

    Ok(helpers::parse_content(&response.text().map_err(Error::ManufResponseRetrieval)?))
}
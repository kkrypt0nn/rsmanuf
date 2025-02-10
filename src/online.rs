use arc_swap::ArcSwap;
use regex::Regex;
use reqwest::StatusCode;
use std::{
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};

use crate::{parse_content, Content};

const TTL: Duration = Duration::from_secs(3600);

static LAST_FETCHED: LazyLock<ArcSwap<SystemTime>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(SystemTime::now())));
static CONTENT: LazyLock<ArcSwap<Content>> = LazyLock::new(|| match fetch_manuf() {
    Ok(content) => {
        LAST_FETCHED.store(Arc::new(SystemTime::now()));
        ArcSwap::new(Arc::new(content))
    }
    Err(e) => {
        eprintln!("{} - fallback to offline index", e);
        LAST_FETCHED.store(Arc::new(SystemTime::now()));
        ArcSwap::new(Arc::new(parse_content(include_str!("manuf.txt"))))
    }
});

pub fn lookup(mac: impl Into<String>) -> Result<String, String> {
    // Update the content if TTL has passed
    let last_fetched = LAST_FETCHED.load();
    if SystemTime::now() > *last_fetched.as_ref() + TTL {
        if let Ok(content) = fetch_manuf() {
            CONTENT.store(Arc::new(content));
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
        }
    }

    let new_mac = mac.into().to_ascii_uppercase().replace("-", ":");

    let regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if regex.find(&new_mac).is_none() {
        return Err(String::from("Invalid MAC address"));
    }

    let content = CONTENT.load();
    if let Some(manuf) = content.data.get(&new_mac) {
        return Ok(manuf.clone());
    }
    if let Some(manuf) = check_slash_28(&new_mac) {
        return Ok(manuf);
    }
    if let Some(manuf) = check_slash_36(&new_mac) {
        return Ok(manuf);
    }

    let prefix = &new_mac[0..8];
    let end = format!("{}:FF:FF:FF", prefix);
    for (prefix, manuf) in content.data.range(prefix.to_string()..=end).rev() {
        if new_mac.starts_with(prefix) {
            return Ok(manuf.clone());
        }
    }

    Ok(String::from("unknown"))
}

fn fetch_manuf() -> Result<Content, String> {
    if let Ok(response) = reqwest::blocking::get(
        "https://raw.githubusercontent.com/kkrypt0nn/rsmanuf/refs/heads/main/src/manuf.txt",
    ) {
        if response.status() != StatusCode::OK {
            return Err(String::from(
                "Failed performing an HTTP request to the online 'manuf.txt' file",
            ));
        }
        if let Ok(source) = response.text() {
            Ok(crate::parse_content(&source))
        } else {
            Err(String::from(
                "Failed getting the content of the online 'manuf.txt' file",
            ))
        }
    } else {
        Err(String::from(
            "Failed performing an HTTP request to the online 'manuf.txt' file",
        ))
    }
}

fn check_slash_28(mac: &str) -> Option<String> {
    let new_mac = format!("{}0:00:00/28", &mac[..10]);
    CONTENT.load().slash_28.get(&new_mac).cloned()
}

fn check_slash_36(mac: &str) -> Option<String> {
    let new_mac = format!("{}0:00/36", &mac[..13]);
    CONTENT.load().slash_36.get(&new_mac).cloned()
}

#[deprecated(
    since = "2025.2.11",
    note = "please use `rsmanuf::online::lookup()` instead"
)]
#[derive(Debug, Clone)]
pub struct Index {}

#[allow(deprecated)]
impl Index {
    #[allow(clippy::new_without_default)]
    #[deprecated(
        since = "2025.2.11",
        note = "please use `rsmanuf::online::lookup()` instead"
    )]
    pub fn new() -> Self {
        Index {}
    }

    #[deprecated(
        since = "2025.2.11",
        note = "please use `rsmanuf::online::lookup()` instead"
    )]
    pub fn search(&mut self, mac: impl Into<String>) -> Result<String, String> {
        lookup(mac)
    }
}

use arc_swap::ArcSwap;
use regex::Regex;
use reqwest::StatusCode;
use std::{
    collections::BTreeMap,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};

use crate::helpers;

const TTL: Duration = Duration::from_secs(3600);

static LAST_FETCHED: LazyLock<ArcSwap<SystemTime>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(SystemTime::now())));
static CONTENT: LazyLock<ArcSwap<BTreeMap<(u64, u8), String>>> =
    LazyLock::new(|| match fetch_manuf() {
        Ok(content) => {
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
            ArcSwap::new(Arc::new(content))
        }
        Err(e) => {
            eprintln!("{} - fallback to offline index", e);
            LAST_FETCHED.store(Arc::new(SystemTime::now()));
            ArcSwap::new(Arc::new(helpers::parse_content(include_str!("manuf.txt"))))
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
    let mac_val = helpers::mac_to_u64(&new_mac).ok_or("Invalid MAC format")?;

    let content = CONTENT.load();
    for &cidr in &[36, 28, 24] {
        let masked = helpers::mask_mac(mac_val, cidr);
        if let Some(m) = content.get(&(masked, cidr)) {
            return Ok(m.clone());
        }
    }

    Ok(String::from("unknown"))
}

fn fetch_manuf() -> Result<BTreeMap<(u64, u8), String>, String> {
    if let Ok(response) = reqwest::blocking::get(
        "https://raw.githubusercontent.com/kkrypt0nn/rsmanuf/refs/heads/main/src/manuf.txt",
    ) {
        if response.status() != StatusCode::OK {
            return Err(String::from(
                "Failed performing an HTTP request to the online 'manuf.txt' file",
            ));
        }
        if let Ok(source) = response.text() {
            Ok(helpers::parse_content(&source))
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

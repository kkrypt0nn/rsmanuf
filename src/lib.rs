use regex::Regex;
use std::{collections::BTreeMap, sync::LazyLock};

#[cfg(feature = "online")]
pub mod online;

struct Content {
    data: BTreeMap<String, String>,
    slash_28: BTreeMap<String, String>,
    slash_36: BTreeMap<String, String>,
}

static CONTENT: LazyLock<Content> = LazyLock::new(|| parse_content(include_str!("manuf.txt")));

pub fn lookup(mac: impl Into<String>) -> Result<String, String> {
    let new_mac = mac.into().to_ascii_uppercase().replace("-", ":");

    let regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if regex.find(new_mac.as_str()).is_none() {
        return Err(String::from("Invalid MAC address"));
    }

    if let Some(res) = check_slash_28(&new_mac) {
        return Ok(res);
    }
    if let Some(res) = check_slash_36(&new_mac) {
        return Ok(res);
    }
    if let Some(manuf) = CONTENT.data.get(&new_mac) {
        return Ok(manuf.clone());
    }

    let prefix = &new_mac[0..8];
    let end = format!("{}:FF:FF:FF", prefix);
    for (prefix, manuf) in CONTENT.data.range(prefix.to_string()..=end).rev() {
        if new_mac.starts_with(prefix) {
            return Ok(manuf.clone());
        }
    }

    Ok(String::from("unknown"))
}

fn parse_content(source: &str) -> Content {
    let mut data = BTreeMap::<String, String>::new();
    let mut slash_28 = BTreeMap::<String, String>::new();
    let mut slash_36 = BTreeMap::<String, String>::new();

    for line in source.lines() {
        let current_line = line.replace("\t\t", "\t");
        let fields = current_line.split('\t').collect::<Vec<&str>>();

        if fields[0].starts_with("#") || line.is_empty() {
            continue;
        }

        let mac = fields[0].to_string();
        let manuf = fields[1].to_string();
        if mac.contains(":00/28") {
            slash_28.insert(mac.clone(), manuf.clone());
        } else if mac.contains(":00/36") {
            slash_36.insert(mac.clone(), manuf.clone());
        }
        data.insert(mac, manuf);
    }

    Content {
        data,
        slash_28,
        slash_36,
    }
}

fn check_slash_28(mac: &str) -> Option<String> {
    let new_mac = format!("{}0:00:00/28", &mac[..10]);
    CONTENT.slash_28.get(&new_mac).cloned()
}

fn check_slash_36(mac: &str) -> Option<String> {
    let new_mac = format!("{}0:00/36", &mac[..13]);
    CONTENT.slash_36.get(&new_mac).cloned()
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
    pub fn search(&self, mac: impl Into<String>) -> Result<String, String> {
        lookup(mac)
    }
}

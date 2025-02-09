use regex::Regex;
use reqwest::StatusCode;
use std::time::{Duration, SystemTime};

use crate::Content;

const TTL: Duration = Duration::from_secs(3600);

#[derive(Debug, Clone)]
pub struct Index {
    content: Content,
    last_fetched: SystemTime,
}

impl Index {
    pub fn new() -> Self {
        match Self::fetch_manuf() {
            Ok(content) => Index {
                content,
                last_fetched: SystemTime::now(),
            },
            Err(e) => {
                eprintln!("{} - fallback to offline index", e);
                Index {
                    content: crate::Index::parse_content(include_str!("manuf.txt")),
                    last_fetched: SystemTime::now(),
                }
            }
        }
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
                Ok(crate::Index::parse_content(&source))
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

    pub fn search(&mut self, mac: impl Into<String>) -> Result<String, String> {
        // Update the content if TTL has passed
        if SystemTime::now() > self.last_fetched + TTL {
            if let Ok(content) = Self::fetch_manuf() {
                self.content = content;
                self.last_fetched = SystemTime::now()
            }
        }

        let mut new_mac: String = mac.into();
        new_mac = new_mac.to_ascii_uppercase().replace("-", ":");

        let regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
        if regex.find(new_mac.as_str()).is_none() {
            return Err(String::from("Invalid MAC address"));
        }

        for (address, manuf) in &self.content.data {
            if new_mac.starts_with(address) {
                // Check if manufacturer is one of those manufacturer that have MACs with /28 or /36
                if manuf == "IEEE Registration Authority" {
                    let check_28 = self.check_slash_28(new_mac.clone());
                    if !check_28.is_empty() {
                        return Ok(check_28);
                    }
                    let check_36 = self.check_slash_36(new_mac.clone());
                    if !check_36.is_empty() {
                        return Ok(check_36);
                    }
                }
                return Ok(manuf.to_string());
            }
        }

        Ok(String::from("unknown"))
    }

    fn check_slash_28(&self, mac: String) -> String {
        let mut new_mac = mac[0..10].to_string();
        new_mac += "0:00:00/28";
        for (address, manuf) in &self.content.slash_28 {
            if address == &*new_mac {
                return manuf.to_string();
            }
        }
        "".to_string()
    }

    fn check_slash_36(&self, mac: String) -> String {
        let mut new_mac = mac[0..13].to_string();
        new_mac += "0:00/36";
        for (address, manuf) in &self.content.slash_36 {
            if address == &*new_mac {
                return manuf.to_string();
            }
        }
        "".to_string()
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

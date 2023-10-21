use regex::Regex;
use std::collections::HashMap;

pub struct Index {
    data: HashMap<String, String>,
    slash_28: HashMap<String, String>,
    slash_36: HashMap<String, String>,
}

impl Index {
    pub fn new() -> Self {
        let mut data = HashMap::<String, String>::new();
        let mut slash_28 = HashMap::<String, String>::new();
        let mut slash_36 = HashMap::<String, String>::new();

        let file = include_str!("manuf.txt");
        for line in file.lines() {
            let current_line = line.replace("\t\t", "\t");
            let fields = current_line.split("\t").collect::<Vec<&str>>();

            if fields[0].starts_with("#") || line == "" {
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

        Index {
            data,
            slash_28,
            slash_36,
        }
    }

    pub fn search(&self, mac: impl Into<String>) -> Result<String, String> {
        let mut new_mac = mac.into();
        new_mac = new_mac.to_ascii_uppercase().replace("-", ":");

        let regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
        if regex.find(new_mac.as_str()).is_none() {
            return Err(String::from("Invalid MAC address"));
        }

        for (address, manuf) in &self.data {
            if new_mac.starts_with(address) {
                // Check if manufacturer is one of those manufacturer that have MACs with /28 or /36
                if manuf == "IEEE Registration Authority" {
                    let check_28 = self.check_slash_28(new_mac.clone());
                    if check_28 != "" {
                        return Ok(check_28);
                    }
                    let check_36 = self.check_slash_36(new_mac.clone());
                    if check_36 != "" {
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
        new_mac = new_mac + "0:00:00/28";
        for (address, manuf) in &self.slash_28 {
            if address == &*new_mac {
                return manuf.to_string();
            }
        }
        "".to_string()
    }

    fn check_slash_36(&self, mac: String) -> String {
        let mut new_mac = mac[0..13].to_string();
        new_mac = new_mac + "0:00/36";
        for (address, manuf) in &self.slash_36 {
            if address == &*new_mac {
                return manuf.to_string();
            }
        }
        "".to_string()
    }
}

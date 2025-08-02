use std::collections::BTreeMap;

pub(crate) fn mac_to_u64(mac: &str) -> Option<u64> {
    let hex: String = mac.split(|c| c == ':').filter(|s| !s.is_empty()).collect();
    let padded_hex = match hex.len() {
        6 => format!("{}000000", hex),
        10 => format!("{}00", hex),
        12 => hex,
        _ => return None,
    };

    u64::from_str_radix(&padded_hex, 16).ok()
}

pub(crate) fn mask_mac(mac: u64, cidr: u8) -> u64 {
    let mask = match cidr {
        24 => 0xFFFFF0000000,
        28 => 0xFFFFFFF00000,
        36 => 0xFFFFFFFFF000,
        _ => 0xFFFFFFFFFFFF,
    };
    mac & mask
}

pub(crate) fn parse_content(source: &str) -> BTreeMap<(u64, u8), String> {
    let mut data = BTreeMap::<(u64, u8), String>::new();

    for line in source.lines() {
        let current_line = line.replace("\t\t", "\t");
        let fields = current_line.split('\t').collect::<Vec<&str>>();

        if fields[0].starts_with("#") || line.is_empty() {
            continue;
        }

        let mac = fields[0];
        let manuf = fields[1].to_string();
        if let Some((mac_prefix, cidr)) = mac.split_once('/') {
            if let Some(mac_val) = mac_to_u64(mac_prefix) {
                let cidr = cidr.parse::<u8>().unwrap_or_default();
                if cidr == 28 || cidr == 36 {
                    data.insert((mask_mac(mac_val, cidr), cidr), manuf.clone());
                    continue;
                }
            }
        } else if let Some(mac_val) = mac_to_u64(mac) {
            let cidr = 24;
            data.insert((mask_mac(mac_val, cidr), cidr), manuf.clone());
        }
    }

    data
}

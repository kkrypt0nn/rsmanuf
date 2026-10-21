use std::collections::BTreeMap;

pub(crate) fn mac_to_u64(mac: &str) -> Option<u64> {
    let hex: String = mac.split(':').filter(|s| !s.is_empty()).collect();
    let padded_hex = match hex.len() {
        6 => format!("{hex}000000"),
        12 => hex,
        _ => return None,
    };

    u64::from_str_radix(&padded_hex, 16).ok()
}

pub(crate) const fn mask_mac(mac: u64, cidr: u8) -> u64 {
    let mask = match cidr {
        24 => 0xFFFFFF000000,
        28 => 0xFFFFFFF00000,
        36 => 0xFFFFFFFFF000,
        _ => 0xFFFFFFFFFFFF,
    };
    mac & mask
}

pub(crate) fn parse_content(source: &str) -> BTreeMap<(u64, u8), String> {
    let mut data = BTreeMap::<(u64, u8), String>::new();

    for line in source.lines() {
        let Some(tab_index) = line.find('\t') else {
            continue;
        };

        let (Some(mac), Some(mut manuf)) = (line.get(0..tab_index), line.get(tab_index + 1..)) else {
            continue;
        };

        if matches!(manuf.as_bytes().first(), Some(b'\t')) {
            manuf = &manuf[1..];
        }

        if let Some((mac_prefix, cidr)) = mac.split_once('/') {
            if let Some(mac_val) = mac_to_u64(mac_prefix) {
                let cidr = cidr.parse::<u8>().unwrap_or_default();

                if cidr == 28 || cidr == 36 {
                    data.insert((mask_mac(mac_val, cidr), cidr), manuf.into());

                    continue;
                }
            }
        } else if let Some(mac_val) = mac_to_u64(mac) {
            let cidr = 24;

            data.insert((mask_mac(mac_val, cidr), cidr), manuf.into());
        }
    }

    data
}

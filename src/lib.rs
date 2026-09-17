use regex::bytes::{Regex, RegexBuilder};
use std::{collections::BTreeMap, sync::LazyLock};

mod error;
mod helpers;
#[cfg(feature = "online")]
#[cfg_attr(docsrs, doc(cfg(feature = "online")))]
pub mod online;

pub use error::Error;

static CONTENT: LazyLock<BTreeMap<(u64, u8), String>> =
    LazyLock::new(|| helpers::parse_content(include_str!("manuf.txt")));
static MAC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
        .unicode(false)
        .build()
        .unwrap()
    );

fn lookup_from_content(content: &BTreeMap<(u64, u8), String>, mac: impl Into<String>) -> Result<Option<String>, Error> {
    let new_mac = mac.into().to_ascii_uppercase().replace('-', ":");

    if MAC_REGEX.find(new_mac.as_bytes()).is_none() {
        return Err(Error::InvalidMacAddress);
    }

    let mac_val = helpers::mac_to_u64(&new_mac).ok_or(Error::InvalidMacAddress)?;

    for cidr in [36, 28, 24] {
        let masked = helpers::mask_mac(mac_val, cidr);

        if let Some(m) = content.get(&(masked, cidr)) {
            return Ok(Some(m.clone()));
        }
    }

    Ok(None)
}

#[inline]
pub fn lookup(mac: impl Into<String>) -> Result<Option<String>, Error> {
    lookup_from_content(&CONTENT, mac)
}
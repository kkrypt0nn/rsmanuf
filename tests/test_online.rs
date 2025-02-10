extern crate rsmanuf;

#[cfg(test)]
mod online_tests {
    #[test]
    fn d_link() {
        let manuf = rsmanuf::online::lookup("C4:A8:1D:73:D7:8C").unwrap();
        assert_eq!(manuf, "D-Link International")
    }

    #[test]
    fn netgear() {
        let manuf = rsmanuf::online::lookup("9C:D3:6D:9A:CA:81").unwrap();
        assert_eq!(manuf, "Netgear")
    }

    #[test]
    fn shanghai_broadwan_communications() {
        let manuf = rsmanuf::online::lookup("40:ED:98:6F:DB:AC").unwrap();
        assert_eq!(manuf, "Shanghai Broadwan Communications Co.,Ltd")
    }

    #[test]
    fn piranha_ems() {
        let manuf = rsmanuf::online::lookup("70:B3:D5:8C:CD:BE").unwrap();
        assert_eq!(manuf, "Piranha EMS Inc.")
    }

    #[test]
    fn ieee_registration_authority() {
        let manuf = rsmanuf::online::lookup("3C:24:F0:F0:BE:CF").unwrap();
        assert_eq!(manuf, "IEEE Registration Authority")
    }

    #[test]
    fn samsung_electronics() {
        let manuf = rsmanuf::online::lookup("24:FC:E5:AD:BB:89").unwrap();
        assert_eq!(manuf, "Samsung Electronics Co.,Ltd")
    }

    #[test]
    fn invalid_address() {
        let manuf = rsmanuf::online::lookup("G4:FC:E5:AD:BB:89").unwrap_err();
        assert_eq!(manuf, "Invalid MAC address")
    }
}

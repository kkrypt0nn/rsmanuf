fn main() {
    match rsmanuf::online::lookup("C4:A8:1D:73:D7:8C") {
        Ok(manuf) => {
            println!("Manufacturer: {}", manuf.as_deref().unwrap_or("Unknown"))
        }
        Err(error) => {
            println!("Error: {error}")
        }
    }
}

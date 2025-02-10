extern crate rsmanuf;

fn main() {
    match rsmanuf::lookup("C4:A8:1D:73:D7:8C") {
        Ok(manuf) => {
            println!("Manufacturer: {}", manuf)
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}

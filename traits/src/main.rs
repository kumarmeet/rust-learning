use std::collections::HashMap;
/*
So we are kind of using the compiler to give us a standard of consistency, to standardize, to formalize
these functionalities so that they are alike.
*/
trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} - {:?}", self.name, self.reservations)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn get_description(&self) -> String {
        format!("{} - {:?}", self.host, self.guests)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn main() {
    let mut handloom = Hotel::new("the handloom");

    handloom.book("john", 10);
    handloom.book("wick", 15);

    let details = handloom.get_description();
    println!("{}", details);

    let mut air_bnb = AirBnb::new("john");
    air_bnb.book("wick", 10);
    println!("{}", air_bnb.get_description());
}

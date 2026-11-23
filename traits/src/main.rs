use std::collections::HashMap;
/*
So we are kind of using the compiler to give us a standard of consistency, to standardize, to formalize
these functionalities so that they are alike.
*/
trait Accommodation {
    // The way this is going to work is that Rust will utilize the method definition from the trait declaration,
    // but only if we do not declare that method when implementing the trait for a given type.
    // So you can think of the methods that we declare here as fallbacks.
    fn get_description(&self) -> String {
        "a wonderful place to stay".to_string() // default implementation
    }
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

    fn summarize(&self) -> String {
        format!(
            "{} - {:?} - {}",
            self.name,
            self.reservations,
            self.get_description() //calling trait method from another method
        )
    }
}

impl Accommodation for Hotel {
    // we can remove this function, as on Accommodation trait we define the get_description method as fallback

    // fn get_description(&self) -> String {
    //     format!("{} - {:?}", self.name, self.reservations)
    // }

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

// 'entity' can be any type as long as it implements the Accommodation trait.
// Rust will not care about the exact type or types passed in, but it will validate that those types implement the Accommodation trait.

fn book_for_one_night(entity: &mut impl Accommodation, name: &str) {
    entity.book(name, 1);
}

// a trait bound requires that a generic type implement a specific trait
// it's an alternate way of above passing trait as func param
fn trait_bound_book_for_one_night<T: Accommodation>(entity: &mut T, name: &str) {
    entity.book(name, 1);
}

fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 4);
    second.book(guest, 8);
}

// this is a syntax of generic trait
fn mix_and_match_another(
    first: &mut impl Accommodation,
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 4);
    second.book(guest, 8);
}

fn main() {
    let mut handloom = Hotel::new("the handloom");

    handloom.book("john", 10);

    book_for_one_night(&mut handloom, "wick");
    trait_bound_book_for_one_night::<Hotel>(&mut handloom, "trait bound 1");

    let details = handloom.summarize();
    println!("{}", details);

    let mut air_bnb = AirBnb::new("air bnb");
    book_for_one_night(&mut air_bnb, "loise");
    trait_bound_book_for_one_night(&mut air_bnb, "trait bound 2");
    println!("{}", air_bnb.get_description());

    mix_and_match(&mut handloom, &mut air_bnb, "kent");
    // above and below both don't any have problem
    mix_and_match::<Hotel, AirBnb>(&mut handloom, &mut air_bnb, "kent");

    mix_and_match_another(&mut handloom, &mut air_bnb, "kent");
}

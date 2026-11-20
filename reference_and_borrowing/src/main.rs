fn main() {
    // mutable and immutable reference
    let mut brand = String::new();

    mutable_reference(&mut brand);
    immutable_reference(&brand);

    // multiple immutable reference
    // rust allows to make multiple immutable references at same time because it guarantees only one owner
    let owner_of_car = String::from("blue");
    let friend1 = &owner_of_car;
    let friend2 = &owner_of_car;

    println!("{} and {} is {}", owner_of_car, friend1, friend2);

    // mutable reference
    // a value can only have a single mutable reference at a time.
    let mut owner_of_car_mutable = String::from("blue");

    /*
        The key takeaway is this: we can have any number of immutable references at the same time, to the same
        value in our program. Or if you have a single mutable reference declared, you cannot have any other
        reference declared to that same value at the same time.
    */
    let friend1 = &mut owner_of_car_mutable;
    let friend3 = &mut owner_of_car_mutable;
    // let friend2 = &owner_of_car_mutable;

    // friend1.push_str("#blue");
    friend3.push_str("#blue");

    // println!("{}", friend1);
    println!("{}", owner_of_car_mutable);

    /*
        This is an advanced feature of Rust called lifetimes.
        And lifetimes basically means that a reference can essentially become invalid before the end of a given scope.
    */
    // rust compiler not allowed to use owner and reference of mutable variable at same time
    // println!("{} and {}", friend1, owner_of_car_mutable);

    // ownership with arrays and tuples
    let registrations = [true, false, true, true];
    let first = registrations[0];

    println!("{first} {registrations:?}");

    // now arrays with heap data
    let currencies = [
        String::from("USD"),
        String::from("EUR"),
        String::from("BTC"),
    ];
    let usd = &currencies[0];
    println!("{usd} {currencies:?}");
}

// immutable_reference
fn immutable_reference(val: &String) {
    println!("{}", val);
}

// mutable reference
fn mutable_reference(val: &mut String) {
    val.push_str("string mutated into the heap");
}

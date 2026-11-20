// The Option enum models a scenario where a type could be a valid value or nothing at all.

// Option::None ->  represents an absent value
// Option::Some(T) -> represents a present value

// The Rust prelude is a collection of named constructs that are made available automatically in every rust program
// You can think of the types, functions, macros, and more that are available to use out of the box in rust
// we can remove the 'Option' keyword and the two colons and accomplish the exact same result.

fn main() {
    let (a, b, c) = (Some(5), Some(String::from("cargo check")), Some(true));
    println!("{:?} {:?} {:?}", a, b, c);

    let d = Option::<u32>::Some(15);
    println!("{:?}", d);

    let e: Option<&str> = Option::None;
    println!("{:?}", e);

    // real example of Option enum
    let business_ideas = [
        String::from("chemist shop"),
        String::from("food shop"),
        String::from("children toy shop"),
    ];

    let chemist = business_ideas.get(0);
    println!("{:?}", chemist);

    let food = business_ideas.get(10);
    println!("{:?}", food);

    // The 'unwrap' method attempts to extract the associated data out of the enum.
    // We get the value by unwrap(), but it's not always the idiomatic Rust way because it's not the safest approach.
    let valid_business = chemist.unwrap();
    println!("{:?}", valid_business);

    // both 'expect' or 'unwrap' suffer from the same problem, which is that they are optimistic.

    // panicked
    // called `Option::unwrap()` on a `None` value
    // let valid_business = food.unwrap();
    // let valid_business = food.expect("not found an element");
    // println!("{:?}", valid_business);

    let business_ideas = [
        String::from("chemist shop"),
        String::from("food shop"),
        String::from("children toy shop"),
    ];

    // Note: Rust implements the Copy trait on the 'Option' enum. like bool, integer, float

    // as you can see when pass the ref Option enum into the check_element_exists() receive a full copy of the enum,
    // so there is not transferring of ownership
    let toy_shop = business_ideas.get(2);
    check_element_exists(toy_shop);

    let food = business_ideas.get(10);
    check_element_exists(food);

    println!("{:?}, {:?}", food, valid_business);

    let has_data = is_item_in_stock(false, true);

    match has_data {
        Some(data) => println!("data: {:?}", data),
        None => println!("data is None"),
    }

    // we can react differently not just based on the variant, but on the associated data that a variant holds.
    match has_data {
        Some(true) => println!("data is true"),
        Some(false) => println!("data is false"),
        None => println!("data is None"),
    }

    //unwrap_or()
    let a = Option::<u32>::Some(5111);
    let b = Option::<u32>::None;

    println!("{:?}, {:?}", a.unwrap_or(0), b.unwrap_or(0));

    // Result enum has two variants Ok and Err, as in rust there is not any exception handling, so in rust uses Result enum
    let ok: Result<i32, &str> = Ok(98);
    println!("{:?}", ok);

    let error: Result<i32, &str> = Err("error");
    println!("{:?}", error);

    let text = "909";
    let parsed_i32 = text.parse::<i32>();

    match parsed_i32 {
        Ok(n) => println!("parsed value -> {n}"),
        Err(e) => println!("parse error -> {e}"),
    }

    let res = divide(10f64, 3f64);
    println!("{:?}", res.is_ok());

    match res {
        Ok(n) => println!("divide result -> {n:.2}"),
        Err(e) => println!("divide error -> {e}"),
    }

    let res = divide(10f64, 0f64);
    println!("{:?}", res.is_ok());
    println!("{:?}", res.is_err());
}

fn check_element_exists<T: std::fmt::Display>(val: Option<&T>) {
    match val {
        Some(s) => println!("{}", s),
        None => println!("element not exist"),
    };
}

// returning an Option enum from a func
fn is_item_in_stock(item_is_in_sys: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_sys && item_is_in_stock {
        Some(true)
    } else if item_is_in_sys {
        Some(false)
    } else {
        None
    }
}

// returning Result enum
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0f64 {
        Err("denominator cannot be zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

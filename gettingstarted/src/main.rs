#![allow(unused_variables)] // for complete file

const TAX_RATE: f64 = 18.25;
type Price = f64;

fn main() {
    let apples: i32 = 50;
    let oranges: i32 = 50 + 10;
    let fruits = apples + oranges;

    // string interpolation
    println!(
        "This year my garden has {apples} apples and {} oranges",
        oranges
    );

    // positional args
    println!(
        "This year my garden has {1} apples and {0} oranges. I can't believe I have {0} oranges.",
        oranges, apples
    );
    println!("The total fruits are {fruits}");

    // in rust by default all var is immutable

    let mut gym_reps = 8;
    println!("I plan to do {gym_reps}");
    gym_reps = 12;
    println!("Now I plan to do {gym_reps}");

    // VARIABLE SHADOWING means redeclaring a variable. The original variable is replaced by the new one

    let grams_of_protein = "100.36";
    println!("string type grams of protein {grams_of_protein}");
    let grams_of_protein = 100.36;
    println!("float type grams of protein {grams_of_protein}");
    let grams_of_protein = 100;
    println!("int 32 type grams of protein {grams_of_protein}");

    //a SCOPE is the boundary or region of code where a name is valid
    let coffee_price: Price = 4.99;

    {
        let cookie_price: Price = 1.99;
        println!("Cookie price {cookie_price}");

        println!("Coffee price {coffee_price}");
    }

    println!("const tax rate {TAX_RATE}");

    // a COMPILER DIRECTIVE (instruction) is an annotation that tell the compiler how to parse the source code
    // #[allow(unused_variables)]
    let mile_race_length = 1600;
}

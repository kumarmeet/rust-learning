fn main() {
    // all primitive data type in rust implements copy trait
    let time = 2025;
    let year = time;

    println!("the time is {time} and year is {year}");

    // string literal
    /*
    Now as it turns out, this type is actually not stored on either the stack or the heap.
    Rather, it is embedded directly into the binary executable that the Rust compiler produces.
    And the reason that is done, because the value is already known at compile time.
    */

    let job_role = "system engineer";

    // string with dynamic memory allocation into the heap memory
    let text = String::new();
    let mut llm_model = String::from("opus");
    llm_model.push_str(" 5.5");
    println!("{}", llm_model);
    println!(
        "length {} and capacity {}",
        llm_model.len(),
        llm_model.capacity()
    );

    llm_model.push_str(" by claude");
    println!("{}", llm_model);

    println!(
        "length {} and capacity {}",
        llm_model.len(),
        llm_model.capacity()
    );

    // moves and ownership (for heap memory)
    // A heap allocated String does not implement the Copy trait.
    let name = String::from("meet kumar");
    let genius = name; // name ownership (variable) has been moved here and new owner is genius
    println!("{}", genius);

    // in rust the drop() method automatically called the scope ends on closing curly braces
    // but user can explicitly call drop() to clear the heap memory and it only clears the heap memory not stack

    let data = String::from("erase this variable memory");
    drop(data);

    // copy
    // Clone trait implement for String type
    let home_address = String::from("127.0.0.1:8888");
    let localhost = home_address.clone();

    println!("{}", localhost);
    println!("{}", home_address);

    // references and borrowing
    // borrowing means we ask the owner to lend us something, and then we use it,
    // and then we promise to give it back to the owner when we're done using it.

    let my_stack_value = 4;
    let my_integer_reference = &my_stack_value;

    // dereference operator (it implements a Display trait in println! macro)
    println!("{}, {}", my_stack_value, *my_integer_reference);

    // Copy trait for primitive type
    let val = 55;
    print_value(val); // eg: let print_val = val;
    println!("{val} still valid");

    // as this in the heap memory, so only one owner at a time and move ownership to full_name() param
    let f_name = String::from("sam altman");
    full_name(f_name); // eg: let full_name = f_name;

    // let mut company_name = String::from("EY");
    // company_rating(&mut company_name);
    // println!("{}", company_name);

    let company_name = String::from("EY");
    company_rating(company_name);
}

// ownership and function params
fn print_value(value: i32) {
    println!("{}", value);
}

fn full_name(user_name: String) {
    println!("{}", user_name);
}

// mutable params
fn company_rating(mut feedback: String) {
    feedback.push_str(" (rating -> 3)");
    println!("{feedback}");
}

// fn company_rating(feedback: &mut String) {
//     feedback.push_str(" (rating -> 3)");
// }

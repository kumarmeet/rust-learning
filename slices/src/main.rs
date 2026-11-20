fn main() {
    //string slice
    let prompt = String::from("What do you want to do?");
    let slice = &prompt[5..];
    println!("{}", slice);

    // let end = prompt.len().saturating_sub(1);

    let slice = &prompt[6..];
    println!("{}", slice);

    let slice = &prompt[..];
    println!("{}", slice);

    // string slices and string literals
    /*
        there is not any dangling reference issue, because while making executable then the user_prompt string literal
         will be stored in to the executable and the next reference &user_prompt[1..] will take reference from the executable memory location
         and hardcoded string will be loaded
    */
    let prompt = {
        let user_prompt = "What do you want to do?";
        &user_prompt[1..]
    };
    println!("{}", prompt);

    // string slice lengths

    /*
        The length of a string slice refers to a count of its bytes, not a count of the number of characters.
    */

    // string slice as func param
    let new_str = String::from("Technical String");
    string_slice(&new_str[1..5]);
    let another_str = "Business Description";
    string_slice(another_str);

    // array slice
    let nums = [1, 2, 32, 43, 521, 74, 255, 39];
    let my_num_slice = &nums[1..5];
    println!("{:?}", my_num_slice);

    // array slice as func param
    array_slice(&nums);
    array_slice(&nums[3..=6]);

    // mutable array slice
    // Rust does not permit mutable slices of strings.
    // If we are borrowing a string slice, we can only do so immutably.
    let mut nums = [1, 2, 32, 43, 521, 74, 255, 39];
    let my_num_slice = &mut nums[1..5];
    my_num_slice[0] = 111;

    println!("my_num_slice -> {:?}", my_num_slice);
    println!("nums -> {:?}", nums);
}

// the  &str supports both option while pass String or &str type
// This feature is called deref coercion.
// And that is when Rust sees a reference and it keeps dereferencing it until it gets to the original value.
fn string_slice(str: &str) {
    println!("{}", str);
}

// This feature is called deref coercion.
fn array_slice(reference: &[i32]) {
    println!("{}", reference.len());
}

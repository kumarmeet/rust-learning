use std::io;

fn main() {
    let mut full_name = "meet".to_string();
    let last_name = "vishwakarma";

    full_name.push(' ');
    full_name.push_str(&last_name);

    println!("{}", full_name);

    let user_id = "testuser".to_string();
    let email_domain = "@domain.com".to_string();

    // user_id + &email_domain, when we use + symbol the rust compiler execute add(&str) behind the scene
    // user_id.add(&email_domain), we can say .add(&str) operator overloaded
    let user_mail = user_id.clone() + &email_domain;
    println!("{}", user_mail);

    // format! macro

    let user_mail = format!("{user_id}{email_domain}");
    println!("{}", user_mail);

    // common string methods

    // 1. Basic string
    let mut text = String::from("Hello Rust Developer");

    println!("Length: {}", text.len());
    println!("Empty: {}", text.is_empty());
    println!("Contains: {}", text.contains("Rust"));
    println!("Starts with: {}", text.starts_with("Hello"));
    println!("Ends with: {}", text.ends_with("Developer"));

    // 2. Conversion
    println!("Uppercase: {}", text.to_uppercase());
    println!("Lowercase: {}", text.to_lowercase());

    let spaces = "   Hello Rust   ";
    println!("Trim: '{}'", spaces.trim());
    println!("Trim start: '{}'", spaces.trim_start());
    println!("Trim end: '{}'", spaces.trim_end());

    // 3. Modification
    text.push('!');
    println!("Push: {}", text);

    text.push_str(" Language");
    println!("Push str: {}", text);

    text.pop();
    println!("Pop: {}", text);

    println!("Replace: {}", text.replace("Rust", "Go"));

    let mut name = String::from("Meet");
    name.insert(0, 'A');
    println!("Insert: {}", name);

    name.insert_str(0, "Hello ");
    println!("Insert str: {}", name);

    name.remove(0);
    println!("Remove: {}", name);

    name.retain(|c| c != 'e');
    println!("Retain: {}", name);

    // 4. Splitting
    let languages = "Rust,Java,Python";

    for language in languages.split(',') {
        println!("Language: {}", language);
    }

    println!(
        "Split whitespace: {:?}",
        "Hello Rust Developer"
            .split_whitespace()
            .collect::<Vec<_>>()
    );

    println!("Split once: {:?}", "name=Meet".split_once('='));

    println!("Split n: {:?}", "a,b,c".splitn(2, ',').collect::<Vec<_>>());

    println!("Lines: {:?}", "Hello\nRust".lines().collect::<Vec<_>>());

    // 5. Searching
    let sentence = "Hello Rust Rust";

    println!("Find: {:?}", sentence.find("Rust"));
    println!("RFind: {:?}", sentence.rfind("Rust"));
    println!(
        "Matches: {:?}",
        sentence.matches("Rust").collect::<Vec<_>>()
    );

    println!(
        "Match indices: {:?}",
        sentence.match_indices("Rust").collect::<Vec<_>>()
    );

    println!("Get: {:?}", sentence.get(0..5));

    // 6. Comparison
    println!("Equal: {}", "Rust".eq("Rust"));

    println!("Ignore case: {}", "RUST".eq_ignore_ascii_case("rust"));

    println!("Compare: {:?}", "Apple".cmp("Banana"));

    // 7. Character iteration
    for ch in "Rust".chars() {
        println!("Character: {}", ch);
    }

    // 8. Byte iteration
    for byte in "ABC".bytes() {
        println!("Byte: {}", byte);
    }

    // 9. Clear
    let mut value = String::from("Hello");
    value.clear();

    println!("After clear: '{}'", value);

    // read input

    let mut name = String::new();
    let mut age = String::new();

    user_input(&mut name, &mut age);
    // let age: u32 = age.trim().parse().unwrap();

    let response = format!("Username: {} and Age: {}", name, age);
    println!("{}", response);
}

fn user_input(name: &mut String, age: &mut String) {
    let inputs = [(stringify!(name), name), (stringify!(age), age)];

    for (field_name, input) in inputs {
        println!("Enter {field_name}: ");
        match io::stdin().read_line(input) {
            Ok(_) => {}
            Err(error) => {
                println!("error: {}", error);
                return;
            }
        }
    }

    *age = age.trim().to_string();
    age.parse::<u32>().unwrap();
}

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process;

fn main() {
    // panic!
    // It's actually a very safe cleanup, so you shouldn't be too scared of the word "panic".
    // Rust will clean everything up and give you the error message on the right hand side.
    // Now, in practice, it is really rare to call this panic! macro directly in your own code.
    // panic!("Something went wrong!");

    // gracefully shutdown the program
    // process::exit(0);

    // eprintln! -> this eprintln! macro prints messages to what's called the standard error.
    // eprintln!("print exit code");

    let mut file_name = String::new();

    read_file(&mut file_name, "Enter the file name you want to open 1:");

    open_file_with_buffer(&file_name);

    // io::stdout().flush().expect("Failed to flush");
    file_name.clear();

    read_file(&mut file_name, "Enter the file name you want to open 2:");

    let data = open_read_file(&mut file_name, "Enter the file name you want to open 3:");

    if let Err(error) = data {
        eprintln!("{}", error);
        process::exit(1);
    }

    println!("{}", data.unwrap());

    // match data {
    //     Ok(content) => println!("{}", content),
    //     Err(error) => {
    //         eprintln!("->>> {}", error);
    //         process::exit(1);
    //     }
    // }

    let mut animals = Vec::<&str>::from(["lion", "zebra", "own"]);
    println!(
        "Animal element length: {:?}",
        option_with_question_mark(&mut animals)
    );
    println!(
        "Animal element length: {:?}",
        option_with_question_mark(&mut animals)
    );
    println!(
        "Animal element length: {:?}",
        option_with_question_mark(&mut animals)
    );
    println!(
        "Animal element length: {:?}",
        option_with_question_mark(&mut animals)
    );
}

fn open_read_file(file_name: &mut String, user_msg: &str) -> Result<String, io::Error> {
    file_name.clear();

    println!("{user_msg}");
    io::stdout().flush()?;

    io::stdin().read_line(file_name)?;

    let mut file_content = String::new();

    File::open(file_name.trim())?.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn option_with_question_mark(animals: &mut Vec<&str>) -> Option<usize> {
    let last_ele = animals.pop()?;
    Some(last_ele.len())
}

fn read_file(file_name: &mut String, user_msg: &str) {
    println!("{user_msg}");
    let file_name_input = io::stdin().read_line(file_name);

    if let Err(err) = file_name_input {
        eprintln!("Error 1: {}", err);
        process::exit(1);
    }
}

fn open_file(file: &str) {
    match fs::read_to_string(file.trim()) {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(error) => {
            eprintln!("ERROR:{}", error);
            process::exit(1);
        }
    }
}

fn open_file_cleaner_way(file: &str) -> Result<String, io::Error> {
    fs::read_to_string(file.trim())
    // match fs::read_to_string(file.trim()) {
    //     Ok(contents) => Ok(contents),
    //     Err(error) => Err(error),
    // }
}

fn open_file_with_buffer(file_name: &str) {
    let file = File::open(file_name.trim());

    match file {
        Ok(mut file) => {
            let mut buffer = [0u8; 1024 * 10]; // read 1mb file only not more than one mb
            let data = file.read(&mut buffer).expect("TODO: panic message");
            println!(
                "opened file: {:?}",
                String::from_utf8_lossy(&buffer[..data])
            );
        }
        Err(e) => {
            eprintln!("could not open file: {}", e);
            process::exit(1);
        }
    }
}

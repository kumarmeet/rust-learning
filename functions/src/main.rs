fn main() {
    let date = open_store(6f64, "bhopal");
    println!("{}", date);
    multiply(3);

    //control flow
    let ans = even_or_odd(10);
    println!("{}", ans);

    // match statement (like switch)
    let evaluation = true;

    match evaluation {
        true => println!("True"),
        false => println!("False"),
    }

    let value = match evaluation {
        true => 10,
        false => 20,
    };

    println!("{}", value);

    let planet = "blue planet";

    match planet {
        "earth" | "blue planet" => println!("Planet earth"),
        "mars" | "red planet" => println!("Planet mars"),
        "jupiter" => println!("Planet jupiter"),
        "uranus" => println!("Planet uranus"),
        "venus" => println!("Planet venus"),
        "saturn" => println!("Planet saturn"),
        "neptune" => println!("Planet neptune"),
        "mercury" => println!("Planet mercury"),
        _ => println!("Planet not found"),
    }

    // conditionals
    let earth_pole = "north pole";

    match earth_pole {
        x if x == "north pole" => println!("north pole"),
        y if y == "south pole" => println!("south pole"),
        _ => println!("Earth pole not found"),
        // _ => unreachable!(),
    }

    let mut counter = 0;

    loop {
        if counter > 10 {
            break;
        }

        println!("{}", counter);
        counter += 1;
    }

    while counter >= 0 {
        println!("{}", counter);
        counter -= 1;
    }
}

fn even_or_odd(num: i32) -> &'static str {
    if num % 2 == 0 { "even" } else { "odd" }
}

fn open_store(budget: f64, place: &str) -> &str {
    println!("opening my medical store at {place} under budget in lakhs {budget}");
    return "january, 2027";
}

// blocks
fn multiply(num: i32) {
    let result = {
        let val = 44;
        val * num
    };

    println!("{}", result);
}

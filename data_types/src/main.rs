fn main() {
    // scalar types -> integer, floating point numbers, boolean and character
    let users_count = 30;
    let total_payments_of_users = 49.00 * users_count as f64;
    let is_fast = true;
    let type_of_users_signature = 'E';

    println!("There are {users_count} \
    users and their total payment of {total_payments_of_users} \
    and that growing fast {is_fast} with the signature {type_of_users_signature}"
    );

    let some_value = 10i8;
    println!("Their value is {some_value}");

    //usize
    let days:usize = 340;
    //isize
    let year:isize = -2026;

    println!("The year is {year}, day is {days}");

    // raw string
    let filepath = r"./root/home/update/file.txt";
    println!("{filepath}");

    //methods
    let value:i32 = -15;
    println!("The value is {}", value.abs());

    let empty_space = "          empty space!          ";
    println!("{}", empty_space.trim());

    let pi = 3.6676f64;
    println!("{}", pi.ceil());
    println!("{}", pi.floor());
    println!("{}", pi.round());
    println!("{}", pi.fract());
    println!("{:.2}", pi);
    println!("{pi:.2}");


    let num = -53i32;
    println!("{}",num.is_positive());
    println!("{}",num.is_negative());

    // compound type

    // arrays
    let numbers:[i32;5] = [1,2,3,4,5];
    println!("{numbers:?}");
    println!("{:?}",numbers);
    println!("{}",numbers.len());
    println!("{:?}",numbers[2]);

    let [a,b,c,d,e] = numbers;
    println!("a={a} b={b} c={c} d={d} e={e}");

    let third_element = numbers[3];
    println!("The third element is {}", third_element);

    let mut exercises = ["dumble press", "incline dumble press", "push ups", "dips"];

    let dumble_press = exercises[0];
    let dips = exercises[3];

    println!("The chest exercises are {1} and {0}", dips, dumble_press);

    exercises[0] = "bench press";

    println!("Chest starts with {}",exercises[0]);


    // tuple
    let person = ("john", 36, true);
    dbg!(person);

    let (name, age, is_active) = person;
    println!("name={name} age={age} is_active={is_active}");

    let first_name = person.0;
    let first_age = person.1;
    let is_active_person = person.2;
    println!("first_name={first_name} first_age={first_age} is_active_person={is_active_person}");

    //ranges and ranges iteration
    let days_in_month = 1..31;
    let days_in_year = 1..=365;

    for i in days_in_month {
        println!("{i}");
    }

    println!("\n");

    for day in days_in_year {
        if day >= 15 && day % 15 == 0 {
            println!("{day}");
        }
    }

    for alpha in 'a'..='d'{
        println!("{alpha}");
    }

    for exercise in exercises{
        println!("{exercise}");
    }

}

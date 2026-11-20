// A generic is a type argument. It's a placeholder for a future type
fn main() {
    println!("Hello, world!");
    println!("{}", identity::<i32>(34));
    println!("{}", identity(true));
    println!("{}", identity(String::from("hello world!")));

    // turbofish operator ::<define type> for e.g. ::<Person>
    let user = identity::<Person>(Person {
        name: String::from("John"),
        age: 32,
    });

    println!("{:?}", user);

    let (m, p) = month_price::<Month, f64>(Month::November, 982f64);
    println!("month is -> {} and price is {p}", m.get_month());

    let find_user = FindByName {
        is_exist: true,
        name: String::from("John"),
    };

    println!("{}", find_user.get_name());

    let mut another_user = FindByName::new(String::from("Doe"), true);
    println!("{}", another_user.get_name());

    another_user.update_name(String::from("testing name"));
    println!("{}", another_user.get_name());
}

#[derive(Debug)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    fn get_month(&self) -> String {
        match self {
            Month::January => String::from("Jan"),
            Month::February => String::from("Feb"),
            Month::March => String::from("Mar"),
            Month::April => String::from("Apr"),
            Month::May => String::from("May"),
            Month::June => String::from("Jun"),
            Month::July => String::from("Jul"),
            Month::August => String::from("Aug"),
            Month::September => String::from("Sep"),
            Month::October => String::from("Oc"),
            Month::November => String::from("Nov"),
            Month::December => String::from("Dec"),
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn identity<T>(x: T) -> T {
    x
}

fn month_price<T, U>(month: T, price: U) -> (T, U) {
    (month, price)
}

struct FindByName<T> {
    name: T,
    is_exist: bool,
}

impl FindByName<String> {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl<T> FindByName<T> {
    fn new(name: T, is_exist: bool) -> FindByName<T> {
        FindByName { name, is_exist }
    }

    fn update_name(&mut self, new_name: T) {
        self.name = new_name;
    }
}

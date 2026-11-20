// An enum is a type that represent a set of possible value.
// An enum is useful when a type represents a value from a limited collection of possible values.

#[derive(Debug)]
enum PaymentMethodType {
    // Behind the scenes, each enum variant becomes a function that constructs an instance of that
    // enum variant and then accepts those respective values.
    UPI(String), // tuple variants with associated data
    DebitCard(),
    CreditCard(String),
    NetBanking(String, String),
}

#[derive(Debug)]
struct DbCreds {
    user_id: String,
    password: String,
}

// struct variant enum
#[derive(Debug)]
enum Database {
    MySql(DbCreds),
    Mongodb { user_id: String, password: String },
}

impl Database {
    fn update_db(self) -> Self {
        match self {
            Self::MySql(mut db) => {
                db.user_id = String::from("root_mysql");
                db.password = String::from("123456789");
                Self::MySql(db)
            }
            Self::Mongodb { .. } => Self::Mongodb {
                user_id: String::from("->root_mongodb"),
                password: String::from("123456789"),
            },
        }
    }
}

// nested enum
#[derive(Debug)]
enum Pass {
    General,
    Silver,
    Golden,
}

#[derive(Debug)]
enum SeatCapacity {
    Four,
    Twenty,
    Thirty,
}

#[derive(Debug)]
enum PublicTransportation {
    RedBus {
        pass: Pass,
        seat_capacity: SeatCapacity,
    },
    BlueBus(SeatCapacity),
    TukTuk(SeatCapacity),
    Auto(SeatCapacity),
}

fn main() {
    let phone_pe = PaymentMethodType::UPI(String::from("+918770851292"));
    println!("{:?}", phone_pe);

    let sbi = PaymentMethodType::NetBanking(String::from("user@sbi.com"), String::from("password"));
    println!("{:?}", sbi);

    let mut db_cred = DbCreds {
        user_id: String::from("root"),
        password: String::from("1234"),
    };

    let mysql = Database::MySql(db_cred);
    println!("{:?}", mysql);

    db_cred = DbCreds {
        user_id: String::from("mongo"),
        password: String::from("1234"),
    };

    let monogo_db = Database::Mongodb {
        user_id: db_cred.user_id,
        password: db_cred.password,
    };
    println!("{:?}", monogo_db);

    let gov_bus = PublicTransportation::RedBus {
        pass: Pass::Golden,
        seat_capacity: SeatCapacity::Thirty,
    };

    let tuktuk = PublicTransportation::TukTuk(SeatCapacity::Four);

    println!("{:?} {:?}", gov_bus, tuktuk);

    let res = get_pass(&Pass::Golden);
    println!("{}", res);

    let db_cred = DbCreds {
        user_id: String::from("root"),
        password: String::from("1234"),
    };

    // let updated_mysql_db = Database::update_db(Database::MySql(db_cred));
    let updated_mysql_db = Database::MySql(DbCreds {
        user_id: String::from(""),
        password: String::from(""),
    })
    .update_db();
    println!("{:?}", updated_mysql_db);

    let updated_monogo_db = Database::update_db(monogo_db);
    println!("{:?}", updated_monogo_db);

    let mongodb_updated_creds = Database::Mongodb {
        user_id: String::from("#####"),
        password: String::from("1234"),
    }
    .update_db();

    println!("->{:?}", mongodb_updated_creds);
}

// match with enums
fn get_pass(val: &Pass) -> String {
    match val {
        Pass::General => "PASS GENERAL".to_string(),
        Pass::Silver => "PASS SILVER".to_string(),
        Pass::Golden => "PASS GOLDEN".to_string(),
    }
}

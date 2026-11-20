#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(Food {
                name: "Uni Sashimi".to_string(),
            })
        } else {
            Some(Food {
                name: "strip steak".to_string(),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if address.is_empty() {
            return Err("no delivery address specified".to_string());
        }

        if self.has_mice_infestation {
            return Err("sorry we have a mice problem".to_string());
        }

        Ok(Food {
            name: "burger".to_string(),
        })
    }
}

fn main() {
    let minto_hall = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    let res = minto_hall.chef_special();
    println!("{:?}", res);

    let res = minto_hall.deliver_burger("bhopal");
    println!("{:?}", res);

    let ashoka = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    let res = ashoka.chef_special();
    println!("{:?}", res);

    let res = ashoka.deliver_burger("bhopal");
    println!("{:?}", res);
}

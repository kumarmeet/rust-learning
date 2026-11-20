use std::collections::{HashMap, HashSet};

fn main() {
    let mut user = HashMap::<String, String>::new();
    // let mut user:HashMap<String, String> = HashMap::new();

    user.insert("name".to_string(), "boston".to_string());
    user.insert("email".to_string(), "boston@test.com".to_string());
    user.insert("dob".to_string(), "12-12-2000".to_string());

    println!("{:#?}", user);

    let mut coords: Vec<(i32, f64)> = Vec::new();

    for x in 1..=6 {
        coords.push((x, x as f64 * 0.5));
    }

    let mut new_cords: HashMap<i32, f64> = coords.iter().copied().collect();
    println!("{:#?}, {:?}", new_cords, coords);

    let val = match new_cords.get(&1) {
        Some(val) => *val,
        None => {
            println!("No value exists");
            0.0
        }
    };

    println!("{:?}", val);

    let fetch_val = new_cords.remove(&3);
    if let Some(fetch_val) = fetch_val {
        println!("key and value removed: {:?}", fetch_val);
    }

    println!("{:#?}", new_cords);

    let value = new_cords[&6];
    println!("{:?}", value);

    let v = new_cords.get(&7).copied().unwrap_or(0.0);
    println!("{:?}", v);

    // overwrite value on key
    new_cords.insert(6, 15.6);
    println!("{:#?}", new_cords);

    let data = new_cords.entry(10).or_insert(24.66);
    *data += 100.0;
    println!("{:#?}", new_cords);

    let nums1 = vec![1, 2, 3, 4, 5, 6];
    let nums2 = vec![1, 3, 5, 6, 7, 8];

    // hash set
    let set1 = nums1.iter().copied().collect::<HashSet<i32>>();
    let set2 = nums2.iter().copied().collect::<HashSet<i32>>();

    println!("set 1 -> {:?}, set 2 -> {:?}", set1, set2);

    let set_union = set1.union(&set2);
    println!("set union -> {:?}", set_union);

    let set_intersection = set1.intersection(&set2);
    println!("set intersection -> {:?}", set_intersection);

    let set_difference = set1.difference(&set2);
    println!("set difference -> {:?}", set_difference);

    let set_symmetric_difference = set1.symmetric_difference(&set2);
    println!("set symmetric_difference -> {:?}", set_symmetric_difference);

    // check no common elements (nothing is common)
    let is_disjoint = set1.is_disjoint(&set2);
    println!("is_disjoint -> {:?}", is_disjoint);

    // let plane = String::from("Boeing");
    // let passenger_count = 160;
    // let travel_options = HashMap::from([(plane, passenger_count)]);
    // println!("travel_options -> {:?}", travel_options);
}

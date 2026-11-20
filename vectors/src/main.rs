fn main() {
    let mut reminder_type = Vec::<&str>::new();
    reminder_type.push("urgent");
    reminder_type.push("moderate");
    reminder_type.push("general");

    println!("{:?}", reminder_type);

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", numbers);

    numbers.insert(1, 800);
    println!("{:?}", numbers);

    let popped_ele = numbers.pop();
    println!("{:?}", popped_ele);

    let removed_ele = numbers.remove(0);
    println!("{:?}", removed_ele);

    let is_ends_with = numbers.ends_with(&[5, 6]);
    println!("{:?}", is_ends_with);

    let vec2 = vec![0, 0, 0];
    numbers.extend(&vec2);
    println!("{:?}", numbers);

    let new_vec = [numbers.clone(), vec2].concat();
    println!("{:?}", new_vec);

    println!("{}", new_vec.len());

    let slice = &new_vec[1..=5];
    println!("{:?}", slice);

    let ele = new_vec.get(0..4);
    let ele = match ele {
        Some(x) => x,
        None => &[],
    };
    println!("{:?}", ele);

    reminder_type[0] = "done";
    println!("{:?}", reminder_type);

    println!(
        "len -> {} capacity -> {}",
        numbers.len(),
        numbers.capacity()
    );
}

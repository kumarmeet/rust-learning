mod inventory;
// use inventory::products::{self, ProductCategory};
// use inventory::MANAGER as INVENTORY_MANAGER;
// use inventory::{Item, MANAGER as INVENTORY_MANAGER, ProductCategory};
// use inventory::products::{Item, ProductCategory};
mod orders;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::{Fake, Faker};
// use orders::MANAGER as ORDER_MANAGER;

// pulling out from lib.rs (library crate)
use project_structure::{INVENTORY_MANAGER, Item, ORDER_MANAGER, ProductCategory};

// the main in src file is a crate root (starting point of the binary crate)
fn main() {
    println!("{} {}", INVENTORY_MANAGER, ORDER_MANAGER);

    // // due to self used on use inventory::products::{self, ProductCategory}, rust can use products submodule
    // let item = products::Item {
    //     name: "ladder".to_string(),
    //     quantity: 2,
    //     category: ProductCategory::Ladders,
    // };

    let item = Item::new("ladder".to_string(), ProductCategory::Ladders, 2);

    dbg!(item);

    let mut fake_item = Faker.fake::<Item>();
    fake_item.name = Name(EN).fake();
    fake_item.category = ProductCategory::Ladders;
    fake_item.quantity = (1..=100).fake();
    println!("{:#?}", fake_item);
}

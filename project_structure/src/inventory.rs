pub mod products;
pub use products::{Item, ProductCategory};
// by default, Rust considers every module item, every name within this module to be private.
pub const FLOOR_SPACE: u32 = 10_000;
pub const MANAGER: &str = "ivan inventory";

fn talk_to_manager() {
    println!("hey manager {MANAGER} how's your coffee");
}

// products is a submodule of inventory

// mod inventory {
//     const FLOOR_SPACE: u32 = 10_000;
//     pub const MANAGER: &str = "ivan inventory";
//
//     #[derive(Debug)]
//     enum ProductCategory {
//         Ladders,
//         Hammer,
//     }
//
//     #[derive(Debug)]
//     struct Item {
//         name: String,
//         category: ProductCategory,
//         quantity: u32,
//     }
//
//     pub fn talk_to_manager() {
//         println!("hey manager {MANAGER} how's your coffee");
//     }
// }

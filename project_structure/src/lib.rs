pub mod inventory;
pub mod orders;
pub use inventory::{Item, MANAGER as INVENTORY_MANAGER, ProductCategory};
pub use orders::MANAGER as ORDER_MANAGER;

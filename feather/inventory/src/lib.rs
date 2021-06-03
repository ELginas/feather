use std::{collections::HashMap, usize};
use std::sync::atomic::AtomicBool;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum InventoryId {
    Block {
        x: u32,
        y: u32,
        z: u32,
    },
    Entity {
        id: u32,
    }
}

type Inventory = ();
type InventoryIndex = u32;

struct InventoryRegistry {
    mapping: HashMap<InventoryId, InventoryIndex>,
    locks: Vec<AtomicBool>,
    inventories: Vec<Option<Inventory>>,
}

impl InventoryRegistry {
    fn get(&self, index: usize) -> Option<&Inventory> {

    }

    fn get_mut(&self, index: usize) -> Option<&mut Inventory> {

    }
}

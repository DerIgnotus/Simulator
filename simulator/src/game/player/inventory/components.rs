use bevy::prelude::*;

pub const HOTBAR_SLOT_COUNT: i32 = 6;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Item {
    pub name: String,
    pub count: i32,
    pub description: String,
    pub image: String,
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub item_entitys: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct Hotbar {
    pub slots: Vec<Slot>,
    pub item_entitys: Vec<Entity>,
}

#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct Slot {
    pub item: Item,
    pub has_item: bool,
}

#[derive(Resource, Default)]
pub struct InventoryActive {
    pub active: bool,
}

#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component, Default)]
pub struct ParentInv {}

#[derive(Component)]
pub struct InventoryHide {}

#[derive(Component)]
pub struct CountText {}

impl Inventory {
    pub fn add_item(&mut self, item: Item) {
        // Check if the item already exists in the inventory
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.name == item.name) {
            existing_item.count += 1; // Increment count if the item is already present
        } else {
            self.items.push(item); // Add the new item if it doesn't exist
        }
    }
}

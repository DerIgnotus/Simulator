use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Item {
    pub name: String,
    pub count: i32,
    pub description: String,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Component, Default)]
pub struct Hotbar {
    pub items: Vec<Item>,
}

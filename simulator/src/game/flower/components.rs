use crate::game::player::inventory::components::Item;
use bevy::prelude::*;
use lazy_static::lazy_static;

#[derive(Component, Clone, Debug)]
pub struct Flower {
    pub flower_type: Type,
    pub stage: u8,
    pub how_many: u8,
    pub timer: Timer,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Type {
    #[default]
    White,
    Blue,
    Red,
}

#[derive(Event)]
pub struct CollectFlower {
    pub what_flower: usize,
    pub flower_pos: Vec3,
    pub other_bonuses: i32,
    pub flower_type: Type,
}

lazy_static! {
    pub static ref ITEM_B: Item = Item {
        name: "Blueberry".to_string(),
        count: 1,
        description: "A blue eddible berry".to_string(),
        image: "No Image".to_string(),
    };
    pub static ref ITEM_R: Item = Item {
        name: "Strawberry".to_string(),
        count: 1,
        description: "A red eddible berry".to_string(),
        image: "No Image".to_string(),
    };
    pub static ref ITEM_W: Item = Item {
        name: "Sunflower".to_string(),
        count: 1,
        description: "A seed to grow a Sunflower".to_string(),
        image: "No Image".to_string(),
    };
}

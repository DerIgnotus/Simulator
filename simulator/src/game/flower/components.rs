use bevy::prelude::*;

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

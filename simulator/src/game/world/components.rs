use bevy::ecs::entity::Entity;
use bevy::{
    math::Vec3,
    prelude::{Component, Resource, Timer},
    transform::components::Transform,
};

pub const FIELD_1_FLOWERS_ROW: i32 = 20;
pub const FIELD_1_FLOWERS_COLUMN: i32 = 13;

pub const FIELD_2_FLOWERS_ROW: i32 = 20;
pub const FIELD_2_FLOWERS_COLUMN: i32 = 13;

#[derive(Component)]
pub struct Ground {}

#[derive(Component, Default, Clone, Debug)]
pub struct FlowerField {
    pub which_field: String,
    pub field_pos: Vec3,
    pub positions: Vec<Vec3>,
    pub flowers: Vec<Flower>,
    pub flower_entities: Vec<Entity>,
}

#[derive(Component, Clone, Debug)]
pub struct Flower {
    pub flower_type: Type,
    pub stage: u8,
    pub how_many: u8,
    pub timer: Timer,
}

#[derive(Clone, Debug)]
pub enum Type {
    White,
    Blue,
    Red,
}

#[derive(Resource, Default, Debug)]
pub struct FlowerFields {
    pub flower_fields: Vec<FlowerField>,
}

#[derive(Resource, Default, Debug)]
pub struct WhatFlowers {
    pub what_flowers_f_1: Vec<Flower>,
    pub what_flowers_f_2: Vec<Flower>,
}

#[derive(Resource, Default, Debug)]
pub struct FlowerTransforms {
    pub flower_transforms: Vec<Transform>,
}

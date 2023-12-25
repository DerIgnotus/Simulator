use bevy::{
    ecs::entity::Entity,
    prelude::{Component, Resource, Vec3},
};
use bevy_time::Timer;

use crate::game::abilities::components::Ability;
use crate::game::flower::components::Type;

#[derive(Component, Default, Clone)]
pub struct Bee {
    pub name: String,
    pub bee_type: Type,
    pub bee_state: BeeState,
    pub found_flower: bool,
    pub which_flower: i32,
    pub damage: i32,
    pub conversion_amount: i32,
    pub gather_amount: i32,
    pub gather_amount_white: i32,
    pub gather_amount_blue: i32,
    pub gather_amount_red: i32,
    pub move_to_point: Vec3,
    pub ability_timer: Timer,

    pub ability_mesh: String,
    pub ability: Ability,
}

#[derive(Component, Default, Clone)]
pub struct Hive {
    pub has_bee: bool,
    pub which_bee: Bee,
}

#[derive(Resource, Default)]
pub struct Hives {
    pub hive_bees: Vec<Hive>,
    pub hives: Vec<Entity>,
}

#[derive(Default, Clone, PartialEq)]
pub enum BeeState {
    #[default]
    FollowPlayer,
    //ConvertPollen,
    CollectPollen,
    //Attack,
}

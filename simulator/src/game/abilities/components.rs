use crate::game::flower::components::Type;
use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Ability {
    pub name: String,
    pub collect_distance: f32,
    pub collect_amount: i32,
    pub collect_amount_white: i32,
    pub collect_amount_blue: i32,
    pub collect_amount_red: i32,
    pub ability_type: Type,
}

#[derive(Event)]
pub struct AbilityEvent {
    pub ability_e: Entity,
    pub ability: Ability,
}

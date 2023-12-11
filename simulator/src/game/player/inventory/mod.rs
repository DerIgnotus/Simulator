use crate::game::player::inventory::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test);
    }
}

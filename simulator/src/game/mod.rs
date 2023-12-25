use crate::game::abilities::AbilityPlugin;
use crate::game::bee::BeePlugin;
use crate::game::flower::FlowerPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::world::WorldPlugin;
use bevy::prelude::*;

pub mod abilities;
pub mod bee;
pub mod flower;
pub mod player;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(BeePlugin)
            .add_plugins(FlowerPlugin)
            .add_plugins(AbilityPlugin);
    }
}

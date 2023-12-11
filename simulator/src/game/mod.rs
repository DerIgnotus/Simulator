use bevy::prelude::*;
use crate::game::player::PlayerPlugin;
use crate::game::world::WorldPlugin;

pub mod player;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(WorldPlugin);

    }
}
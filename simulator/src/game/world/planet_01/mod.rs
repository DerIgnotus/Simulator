use crate::game::world::planet_01::systems::*;
use bevy::prelude::*;

use crate::game::player::systems::setup;

pub mod systems;

pub struct Planet01Plugin;

impl Plugin for Planet01Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world.after(setup));
    }
}

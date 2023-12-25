use bevy::prelude::*;

use crate::game::player::systems::setup;

pub mod components;
mod systems;
use self::systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui.after(setup))
            .add_systems(Update, update_ui);
    }
}

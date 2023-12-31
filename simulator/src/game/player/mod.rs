use crate::game::player::systems::*;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

use self::components::{Collect, Honey, InField, OnGround, Pollen};
use self::inventory::InventoryPlugin;
use self::ui::UiPlugin;
use crate::game::world::planet_01::systems::setup_world;

pub mod components;
pub mod inventory;
pub mod systems;
pub mod ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnGround>()
            .init_resource::<InField>()
            .init_resource::<Honey>()
            .init_resource::<Pollen>()
            .add_event::<Collect>()
            .add_plugins(ThirdPersonCameraPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(UiPlugin)
            .add_systems(PreStartup, setup)
            .add_systems(Update, (player_movement).after(setup))
            .add_systems(Update, (is_grounded, is_in_field).after(player_movement))
            .add_systems(
                Update,
                (collector_swinging_system, use_ability, collect_flowers).after(setup_world),
            );
    }
}

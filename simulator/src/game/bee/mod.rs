use self::{components::Hives, systems::*};
use bevy::prelude::*;
pub mod components;
mod systems;

pub struct BeePlugin;

impl Plugin for BeePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hives>().add_systems(
            Update,
            (
                spawn_bee_with_key,
                follow_player,
                what_bee_do,
                bee_collect_pollen,
                bee_ability,
            ),
        );
    }
}

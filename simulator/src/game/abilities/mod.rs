use bevy::prelude::*;

use self::components::AbilityEvent;
use self::systems::*;
pub mod components;
mod systems;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityEvent>()
            .add_systems(Update, ability_1);
    }
}

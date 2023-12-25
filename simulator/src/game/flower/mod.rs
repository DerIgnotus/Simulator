use bevy::prelude::*;

use self::components::CollectFlower;
use self::systems::*;
pub mod components;
pub mod systems;

pub struct FlowerPlugin;

impl Plugin for FlowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectFlower>()
            .add_systems(Update, (collect_flower, flower_text_update));
    }
}

use crate::game::world::planet_01::Planet01Plugin;
use bevy::prelude::*;

use self::components::{FlowerFields, FlowerTransforms, WhatFlowers};

pub mod components;
pub mod planet_01;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlowerFields>()
            .init_resource::<WhatFlowers>()
            .init_resource::<FlowerTransforms>()
            .add_plugins(Planet01Plugin);
    }
}

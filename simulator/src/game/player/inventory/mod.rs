use crate::game::player::inventory::systems::*;
use bevy::prelude::*;

use self::components::{Hotbar, Inventory, InventoryActive};

pub mod components;
pub mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryActive>()
            .init_resource::<Inventory>()
            .init_resource::<Hotbar>()
            .add_systems(Startup, (inventory_hotbar_setup, inventory_setup))
            .add_systems(
                Update,
                (
                    update_hotbar,
                    display_inventory_items,
                    toggle_inventory,
                    mouse_scroll,
                ),
            );
    }
}

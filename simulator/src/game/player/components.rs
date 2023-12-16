use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Resource, Default)]
pub struct OnGround {
    pub is_grounded: bool,
}

#[derive(Resource, Default)]
pub struct SwingTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Component, Default, Clone)]
pub struct Collector {
    pub name: String,
    pub collect_amount: u32,
    pub collect_amount_white: u8,
    pub collect_amount_blue: u8,
    pub collect_amount_red: u8,
    pub mesh_handle: Handle<Scene>,

    // Swing
    pub can_swing: bool,
}

#[derive(Event)]
pub struct Collect {}

#[derive(Resource, Default)]
pub struct InField {
    pub is_in_field: bool,
    pub which_field: String,
}

#[derive(Resource, Default)]
pub struct Honey {
    pub money: i64,
}

#[derive(Resource, Default)]
pub struct Pollen {
    pub pollen_in_backpack: i64,
}

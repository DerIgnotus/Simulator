use crate::systems::exit_system;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::WindowTheme;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use game::GamePlugin;

mod game;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulator".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                position: WindowPosition::Centered(MonitorSelection::Current),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(GamePlugin)
        .add_systems(Update, exit_system)
        .run();
}

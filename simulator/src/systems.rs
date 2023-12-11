use bevy::prelude::*;
use bevy::app::AppExit;

pub fn exit_system(mut exit: EventWriter<AppExit>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        exit.send(AppExit);
    }
}
use bevy::prelude::*;

use crate::game::player::components::Pollen;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "Pollen: ".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::YELLOW, // Change color if needed
                        ..Default::default()
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update_ui(mut query: Query<&mut Text>, pollen: Res<Pollen>) {
    for mut text in &mut query.iter_mut() {
        // Update the Text value to display the current Pollen resource value
        if let Some(section) = text.sections.get_mut(1) {
            section.value = pollen.pollen_in_backpack.to_string(); // Update with Pollen resource value
        }
    }
}

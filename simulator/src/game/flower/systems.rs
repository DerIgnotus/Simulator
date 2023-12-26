use crate::game::flower::components::*;
use crate::game::player::components::*;
use crate::game::player::inventory::systems::*;
use crate::game::player::ui::components::*;
use crate::game::world::components::FlowerFields;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use rand::*;

use super::components::{CollectFlower, Type};

pub fn collect_flower(
    in_field: Res<InField>,
    mut flowerfields: ResMut<FlowerFields>,
    mut event_reader: EventReader<CollectFlower>,
    mut pollen_res: ResMut<Pollen>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        let what_flower = event.what_flower;
        let flower_pos = event.flower_pos;
        let other_bonuses = event.other_bonuses;

        let which_field = in_field.which_field_int as usize;
        let current_field = &mut flowerfields.flower_fields[which_field];

        pollen_res.pollen_in_backpack += other_bonuses as i64;

        current_field.flowers[what_flower].stage -= 1;

        flower_text_spawn(flower_pos, &mut commands, other_bonuses);

        let mut rng = thread_rng();
        let give_berry = rng.gen_range(0..100);

        if give_berry > 70 {
            let mut inventory = GLOBAL_INVENTORY.lock().unwrap();

            if event.flower_type == Type::Blue {
                inventory.add_item(ITEM_B.clone());
            } else if event.flower_type == Type::Red {
                inventory.add_item(ITEM_R.clone());
            } else {
                inventory.add_item(ITEM_W.clone());
            }
        }
    }
}

pub fn flower_text_spawn(position: Vec3, commands: &mut Commands, how_much_pollen: i32) {
    let spawn_pos = Vec3::new(position.x, position.y + 1.0, position.z);

    commands.spawn((
        BillboardTextBundle {
            transform: Transform::from_translation(spawn_pos).with_scale(TEXT_SCALE),
            text: Text::from_sections([TextSection {
                value: how_much_pollen.to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::ORANGE,
                    ..Default::default()
                },
            }])
            .with_alignment(TextAlignment::Center),
            ..default()
        },
        ThisText {},
    ));
}

pub fn flower_text_update(
    time: Res<Time>,
    bill_board_q: Query<Entity, (With<ThisText>, Without<Player>)>,
    mut bill_board_t_q: Query<&mut Transform, (With<ThisText>, Without<Player>)>,
    mut commands: Commands,
) {
    for entity in bill_board_q.iter() {
        let mut transform_t = bill_board_t_q.get_mut(entity).unwrap();
        transform_t.translation.y += time.delta_seconds() * 0.5; // Adjust the value as needed

        if transform_t.translation.y >= 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

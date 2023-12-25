use crate::game::abilities::components::*;
use crate::game::flower::components::*;
use crate::game::player::components::*;
use crate::game::world::components::FlowerFields;
use bevy::prelude::*;

pub fn ability_1(
    which_field: Res<InField>,
    mut flowerfields: ResMut<FlowerFields>,
    ability_t_q: Query<&Transform, (With<Ability>, Without<Player>)>,
    mut commands: Commands,
    mut event_writer: EventWriter<CollectFlower>,
    mut event_reader: EventReader<AbilityEvent>,
) {
    for ability_event in event_reader.read() {
        let ability_e = ability_event.ability_e;
        let ability = &ability_event.ability;
        let field = &mut flowerfields.flower_fields[which_field.which_field_int as usize];
        let ability_transform = ability_t_q.get(ability_e).unwrap();

        for (count, flower) in field.flowers.iter_mut().enumerate() {
            let flower_pos_t = field.positions[count] + field.field_pos;
            let distance = ability_transform.translation.distance(flower_pos_t);

            if (ability.collect_distance >= distance) && (flower.stage > 0) {
                let times_cause_color: i32;
                let flower_t: Type = match flower.flower_type {
                    Type::White => {
                        times_cause_color = ability.collect_amount_white;
                        Type::White
                    }
                    Type::Blue => {
                        times_cause_color = ability.collect_amount_blue;
                        Type::Blue
                    }
                    Type::Red => {
                        times_cause_color = ability.collect_amount_blue;
                        Type::Red
                    }
                };

                let how_much_pollen =
                    flower.how_many as i32 * ability.collect_amount * times_cause_color;

                event_writer.send(CollectFlower {
                    what_flower: count,
                    flower_pos: flower_pos_t,
                    other_bonuses: how_much_pollen,
                    flower_type: flower_t,
                });

                commands.entity(ability_e).despawn_recursive();
            }
        }
    }
}

#![allow(clippy::type_complexity)]

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};

use super::components::{Bee, BeeState, Hive, Hives};
use crate::game::abilities::components::Ability;
use crate::game::flower::components::{CollectFlower, Type};
use crate::game::player::components::{InField, Player};
use crate::game::world::components::FlowerFields;

pub fn spawn_bee_with_key(
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut hives_res: ResMut<Hives>,
    hive_q: Query<&GlobalTransform, With<Hive>>,
) {
    let bee_meshes: Vec<Handle<Scene>> = vec![asset_server.load("meshes/normal_bee.glb#Scene0")];
    let abilitys: Vec<Ability> = vec![Ability {
        name: "Simple Explosion".to_string(),
        ability_type: Type::White,
        collect_distance: 2.0,
        collect_amount: 10,
        collect_amount_white: 2,
        collect_amount_blue: 1,
        collect_amount_red: 1,
    }];

    let mut rng = thread_rng();
    let timer = rng.gen_range(8.0..20.0);

    let bees = vec![Bee {
        name: "Normal Bee".to_string(),
        bee_type: Type::White,
        bee_state: BeeState::FollowPlayer,
        found_flower: false,
        which_flower: 0,
        damage: 1,
        conversion_amount: 50,
        gather_amount: 10,
        gather_amount_white: 1,
        gather_amount_blue: 1,
        gather_amount_red: 1,
        ability_timer: Timer::from_seconds(timer, TimerMode::Once),

        move_to_point: Vec3::new(0.0, 0.0, 0.0),
        ability_mesh: "meshes/ability_1.glb#Scene0".to_string(),
        ability: abilitys[0].clone(),
    }];

    if keycode.just_pressed(KeyCode::Key1) {
        for count in 0..hives_res.hive_bees.len() {
            let hive = &mut hives_res.hive_bees[count];
            if !hive.has_bee {
                hive.has_bee = true;
                hive.which_bee = bees[0].clone();
                let entity = hives_res.hives[count];

                let global_transform = hive_q.get(entity).unwrap();
                let transform = global_transform.compute_transform();

                commands.spawn(SceneBundle {
                    scene: bee_meshes[0].clone(),
                    transform: Transform {
                        translation: Vec3 {
                            x: transform.translation.x,
                            y: transform.translation.y - 1.2,
                            z: transform.translation.z,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });

                let mut rng = thread_rng();
                let random_x = rng.gen_range(-7.0..7.0);
                let random_z = rng.gen_range(-7.0..7.0);

                let mut bee_c = bees[0].clone();
                bee_c.move_to_point = Vec3::new(random_x, 0.0, random_z);

                commands.spawn((
                    SceneBundle {
                        scene: bee_meshes[0].clone(),
                        ..Default::default()
                    },
                    bee_c,
                ));
                break;
            }
        }
    }
}

pub fn what_bee_do(
    player_in_field: Res<InField>,
    mut bee_q: Query<&mut Bee, (With<Bee>, Without<Player>)>,
) {
    for mut bee in bee_q.iter_mut() {
        if player_in_field.is_in_field {
            bee.bee_state = BeeState::CollectPollen;
        } else {
            bee.bee_state = BeeState::FollowPlayer;
        }
    }
}

pub fn follow_player(
    mut bee_q: Query<(&mut Bee, Entity), (With<Bee>, Without<Player>)>,
    mut bee_transform_q: Query<&mut Transform, (With<Bee>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
) {
    for (bee, bee_entity) in bee_q.iter_mut() {
        if bee.bee_state == BeeState::FollowPlayer {
            let player_transform = player_q.single();

            let mut bee_transform = bee_transform_q.get_mut(bee_entity).unwrap();

            let move_point = Vec3::new(
                player_transform.translation.x + bee.move_to_point.x,
                0.0,
                player_transform.translation.z + bee.move_to_point.z,
            );

            let distance = bee_transform.translation.distance(move_point);

            bee_transform.look_at(move_point, Vec3::Y);
            let forward = bee_transform.forward();

            if distance > 0.2 {
                bee_transform.translation += forward * 0.1;
            }
        }
    }
}

pub fn bee_collect_pollen(
    mut bee_q: Query<(&mut Bee, Entity), (With<Bee>, Without<Player>)>,
    in_field: Res<InField>,
    mut flowerfields: ResMut<FlowerFields>,
    mut bee_transform_q: Query<&mut Transform, (With<Bee>, Without<Player>)>,
    mut event_writer: EventWriter<CollectFlower>,
) {
    for (mut bee, bee_entity) in bee_q.iter_mut() {
        if bee.bee_state == BeeState::CollectPollen {
            let which_field = in_field.which_field_int as usize;
            let which_flowerfield = &mut flowerfields.flower_fields[which_field];
            let mut bee_transform = bee_transform_q.get_mut(bee_entity).unwrap();
            if !bee.found_flower {
                let mut rng = rand::thread_rng();

                bee.which_flower = rng.gen_range(0..which_flowerfield.flowers.len() as i32);
            }
            bee.found_flower = true;

            let flower_c = &mut which_flowerfield.flowers[bee.which_flower as usize];
            let _flower_e = &which_flowerfield.flower_entities[bee.which_flower as usize];

            let flower_transform = which_flowerfield.positions[bee.which_flower as usize]
                + which_flowerfield.field_pos;

            let distance = flower_transform.distance(bee_transform.translation);

            if distance > 0.1 {
                bee_transform.look_at(flower_transform, Vec3::Y);
                let forward = bee_transform.forward();
                bee_transform.translation += forward * 0.03;
            } else if flower_c.stage > 0 {
                let times_cause_color: i32;
                let flower_t: Type = match flower_c.flower_type {
                    Type::White => {
                        times_cause_color = bee.gather_amount_white;
                        Type::White
                    }
                    Type::Blue => {
                        times_cause_color = bee.gather_amount_blue;
                        Type::Blue
                    }
                    Type::Red => {
                        times_cause_color = bee.gather_amount_red;
                        Type::Red
                    }
                };
                let give_pollen: i64 =
                    flower_c.how_many as i64 * bee.gather_amount as i64 * times_cause_color as i64;

                event_writer.send(CollectFlower {
                    what_flower: bee.which_flower as usize,
                    flower_pos: flower_transform,
                    other_bonuses: give_pollen as i32,
                    flower_type: flower_t,
                });
                bee.found_flower = false;
            }
        }
    }
}

pub fn bee_ability(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bee_q: Query<(&mut Bee, &Transform), (With<Bee>, Without<Player>)>,
) {
    for (mut bee, bee_transform) in bee_q.iter_mut() {
        if bee.bee_state == BeeState::CollectPollen {
            bee.ability_timer.tick(time.delta());
            if bee.ability_timer.finished() {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load(bee.ability_mesh.clone()),
                        transform: Transform::from_xyz(
                            bee_transform.translation.x,
                            bee_transform.translation.y + 0.5,
                            bee_transform.translation.z,
                        ),
                        ..Default::default()
                    },
                    Collider::cuboid(0.5, 0.5, 0.1),
                    bee.ability.clone(),
                ));
                let mut rng = thread_rng();
                let timer_dur = rng.gen_range(6.0..20.0);
                let duration = Duration::new(timer_dur as u64, timer_dur as u32);
                bee.ability_timer.set_duration(duration);
                bee.ability_timer.reset();
            }
        }
    }
}

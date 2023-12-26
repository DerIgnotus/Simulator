#![allow(clippy::type_complexity)]

use crate::game::abilities::components::*;
use crate::game::flower::components::{CollectFlower, Type};
use crate::game::{
    player::components::*,
    world::components::{FlowerField, FlowerFields, Ground},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

pub fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let player_mesh: Handle<Scene> = asset_server.load("meshes/player.glb#Scene0");

    let tools: Vec<Collector> = vec![Collector {
        name: "Shovel".to_string(),
        collect_amount: 1,
        ability_collect_amount: 3.0,
        collect_amount_white: 1,
        collect_amount_blue: 1,
        collect_amount_red: 1,
        mesh_handle: asset_server.load("meshes/shovel.glb#Scene0"),
        until_ability: 10,
        can_swing: true,
        swing_range: 1.65,
        ability_range: 5.0,
    }];

    let player = commands
        .spawn((
            SceneBundle {
                scene: player_mesh,
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                ..Default::default()
            },
            Player {},
            ThirdPersonCameraTarget,
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
            Sleeping {
                sleeping: false,
                ..Default::default()
            },
            GravityScale(1.0),
            ExternalImpulse {
                impulse: Vec3::new(0.0, 0.0, 0.0),
                torque_impulse: Vec3::new(0.0, 0.0, 0.0),
            },
            ActiveEvents::CONTACT_FORCE_EVENTS,
            (LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y),
            Collider::capsule(
                Vec3 {
                    x: (0.0),
                    y: (0.3),
                    z: (0.0),
                },
                Vec3 {
                    x: (0.0),
                    y: (1.2),
                    z: (0.0),
                },
                0.85,
            ),
            KinematicCharacterController {
                //offset: CharacterLength::Absolute(0.02),
                max_slope_climb_angle: 45.0_f32.to_radians(),
                min_slope_slide_angle: 20.0_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.5),
                    min_width: CharacterLength::Absolute(0.2),
                    include_dynamic_bodies: true,
                }),
                //snap_to_ground: Some(CharacterLength::Absolute(0.2)),
                ..Default::default()
            },
            Speed(8.0),
        ))
        .with_children(|p| {
            p.spawn((
                SceneBundle {
                    scene: tools[0].mesh_handle.clone(),
                    transform: Transform::from_xyz(0.7, 0.3, 0.0),
                    ..Default::default()
                },
                tools[0].clone(),
            ));
        })
        .insert(Ccd::enabled())
        .id();

    println!("Player Id: {:?}", player);

    commands.insert_resource(SwingTimer {
        // create the repeating timer
        timer: Timer::from_seconds(1.5, TimerMode::Once),
    });

    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load("meshes/shovel.glb#Animation1"),
        asset_server.load("meshes/shovel.glb#Animation0"),
    ]));

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            mouse_sensitivity: 10.0,
            cursor_lock_key: KeyCode::F,
            zoom: Zoom::new(2.0, 15.0),
            offset_enabled: true,
            offset: Offset::new(0.0, 1.6),
            ..Default::default()
        },
    );
    commands.spawn(camera);
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    on_ground: Res<OnGround>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = cam_q.get_single().unwrap();

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        if keys.pressed(KeyCode::Space) && on_ground.is_grounded {
            for mut ext_impulse in ext_impulses.iter_mut() {
                ext_impulse.impulse = Vec3::new(0.0, 15.0, 0.0);
                ext_impulse.torque_impulse = Vec3::new(0.0, 0.5, 0.0);
            }
        }

        let mut movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();

        movement.y = 0.0;
        direction.y = 0.0;

        player_transform.translation += movement;

        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}

/* A system that displays the events. */
pub fn is_grounded(
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut sleeping_q: Query<&mut Sleeping, With<Player>>,
    mut on_ground: ResMut<OnGround>,
    player_q: Query<Entity, With<Player>>,
    grounds_q: Query<Entity, With<Ground>>,
) {
    for mut sleeping in sleeping_q.iter_mut() {
        sleeping.sleeping = false;
        let player = player_q.single();

        on_ground.is_grounded = false;

        for contact_force_event in contact_force_events.read() {
            let entity_1 = contact_force_event.collider1;
            let entity_2 = contact_force_event.collider2;

            for entity in grounds_q.iter() {
                if (entity_1 == player && entity_2 == entity)
                    || (entity_1 == entity && entity_2 == player)
                {
                    on_ground.is_grounded = true;
                    return; // Exit the loop once grounded
                }
            }
        }
    }
}

pub fn is_in_field(
    player_q: Query<Entity, With<Player>>,
    field_q: Query<(Entity, &FlowerField), With<FlowerField>>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut in_field: ResMut<InField>,
    mut sleeping_q: Query<&mut Sleeping, With<Player>>,
) {
    for mut sleeping in sleeping_q.iter_mut() {
        sleeping.sleeping = false;
        let player = player_q.single();

        in_field.is_in_field = false;

        for contact_force_event in contact_force_events.read() {
            let entity_1 = contact_force_event.collider1;
            let entity_2 = contact_force_event.collider2;

            for (entity, field) in field_q.iter() {
                if (entity_1 == player && entity_2 == entity)
                    || (entity_1 == entity && entity_2 == player)
                {
                    in_field.is_in_field = true;

                    match field.which_field.as_str() {
                        "field_1" => (
                            in_field.which_field = field.which_field.clone(),
                            in_field.which_field_int = 0,
                        ),
                        "field_2" => (
                            in_field.which_field = field.which_field.clone(),
                            in_field.which_field_int = 1,
                        ),
                        _ => (println!("Wrong FlowerField Name!"), println!("test")),
                    };
                    return;
                }
            }
        }
    }
}

pub fn collector_swinging_system(
    time: Res<Time>,
    input: Res<Input<MouseButton>>,
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(&mut Collector, &mut Transform)>,
    mut player_query: Query<&mut AnimationPlayer>,
    mut swing_timer: ResMut<SwingTimer>,
    mut event_writer: EventWriter<Collect>,
) {
    for (mut collector, _transform) in query.iter_mut() {
        if input.just_pressed(MouseButton::Left) && collector.can_swing {
            collector.until_ability -= 1;
            for mut player in &mut player_query {
                if collector.until_ability == 0 {
                    player.play(player_animations.0[1].clone_weak());
                    player.replay();
                } else {
                    player.play(player_animations.0[0].clone_weak());
                    player.replay();
                }
            }

            event_writer.send(Collect {});

            collector.can_swing = false;
            swing_timer.timer.reset();
        }

        swing_timer.timer.tick(time.delta());
        collector.can_swing = swing_timer.timer.finished();
    }
}

pub fn collect_flowers(
    in_field: Res<InField>,
    player_q: Query<&Transform, With<Player>>,
    mut tool_q: Query<&mut Collector>,
    mut flowerfields: ResMut<FlowerFields>,
    mut event_reader: EventReader<Collect>,
    mut event_writer: EventWriter<CollectFlower>,
) {
    for _event in event_reader.read() {
        if in_field.is_in_field {
            // === Variables ===

            let which_field = in_field.which_field_int as usize;
            let player_position = player_q.single().translation;
            let mut vec_i_want: Vec3 = player_position;
            vec_i_want.y -= 0.5;
            let current_field = &mut flowerfields.flower_fields[which_field];
            let poses = &current_field.positions;
            let field_position = &current_field.field_pos;
            let mut tool = tool_q.single_mut();

            let tool_ability_bonus;
            let tool_range;

            if tool.until_ability == 0 {
                tool_range = tool.ability_range;
                tool_ability_bonus = tool.ability_collect_amount;
                tool.until_ability = 10;
            } else {
                tool_range = tool.swing_range;
                tool_ability_bonus = 1.0;
            }

            for (what_flower_t, &position) in poses.iter().enumerate() {
                let flower_pos_t = position + *field_position;
                let distance = vec_i_want.distance(flower_pos_t);

                if (distance < tool_range) && (current_field.flowers[what_flower_t].stage > 0) {
                    let times_cause_color: i32;
                    let flower_t: Type = match current_field.flowers[what_flower_t].flower_type {
                        Type::White => {
                            times_cause_color = tool.collect_amount_white as i32;
                            Type::White
                        }
                        Type::Blue => {
                            times_cause_color = tool.collect_amount_blue as i32;
                            Type::Blue
                        }
                        Type::Red => {
                            times_cause_color = tool.collect_amount_blue as i32;
                            Type::Red
                        }
                    };

                    let how_much_pollen = current_field.flowers[what_flower_t].how_many as i32
                        * tool.collect_amount as i32
                        * times_cause_color
                        * tool_ability_bonus as i32;

                    event_writer.send(CollectFlower {
                        what_flower: what_flower_t,
                        flower_pos: flower_pos_t,
                        other_bonuses: how_much_pollen,
                        flower_type: flower_t,
                    });
                }
            }
        }
    }
}

pub fn use_ability(
    mut contact_force_events: EventReader<ContactForceEvent>,
    player_q: Query<Entity, With<Player>>,
    ability_q: Query<(Entity, &Ability), (With<Ability>, Without<Player>)>,
    //which_field: Res<InField>,
    //mut flowerfields: ResMut<FlowerFields>,
    //ability_t_q: Query<&Transform, (With<Ability>, Without<Player>)>,
    //mut pollen_res: ResMut<Pollen>,
    //mut commands: Commands,
    mut event_writer: EventWriter<AbilityEvent>,
) {
    let player = player_q.single();

    for contact_force_event in contact_force_events.read() {
        let entity_1 = contact_force_event.collider1;
        let entity_2 = contact_force_event.collider2;

        for (ability_e_t, ability) in ability_q.iter() {
            if (entity_1 == player) && (entity_2 == ability_e_t) {
                event_writer.send(AbilityEvent {
                    ability_e: ability_e_t,
                    ability: ability.clone(),
                });
            }
        }
    }
}

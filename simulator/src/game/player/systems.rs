#![allow(unused_assignments)]
#![allow(clippy::too_many_arguments)]

use crate::game::{
    player::components::*,
    world::components::{Flower, FlowerField, FlowerFields, Ground, Type},
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
        collect_amount_white: 1,
        collect_amount_blue: 1,
        collect_amount_red: 1,
        mesh_handle: asset_server.load("meshes/shovel.glb#Scene0"),
        can_swing: true,
    }];

    let player = commands
        .spawn((
            SceneBundle {
                scene: player_mesh,
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
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
            Speed(5.0),
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
        asset_server.load("meshes/shovel.glb#Animation0")
    ]));

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            mouse_sensitivity: 6.0,
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
                        "field_1" => in_field.which_field = field.which_field.clone(),
                        "field_2" => in_field.which_field = field.which_field.clone(),
                        _ => println!("Wrong FlowerField Name!"),
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
            for mut player in &mut player_query {
                player.play(player_animations.0[0].clone_weak());
                player.replay();
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
    tool_q: Query<&Collector>,
    flower_que: Query<&Flower, With<Flower>>,
    mut flowerfields: ResMut<FlowerFields>,
    mut event_reader: EventReader<Collect>,
    mut pollen_res: ResMut<Pollen>,
    mut flower_q: Query<&mut Transform, (With<Flower>, Without<Player>)>,
) {
    for _event in event_reader.read() {
        let mut which_flower_id_plus = 0;
        let mut which_field: usize = 0;
        if in_field.is_in_field {
            match in_field.which_field.as_str() {
                "field_1" => {
                    which_field = 0;
                    which_flower_id_plus = 0
                }
                "field_2" => {
                    which_field = 1;
                    which_flower_id_plus = 260
                }
                _ => println!(
                    "Field {} has a typo or I forgot to add it!",
                    in_field.which_field
                ),
            };

            let player_position = player_q.single().translation;

            let mut vec_i_want: Vec3 = player_position;
            let poses = flowerfields.flower_fields[which_field].positions.clone();
            vec_i_want.y -= 0.5;

            for (what_flower, &position) in poses.iter().enumerate() {
                let field_position = flowerfields.flower_fields[which_field].field_pos;

                let distance = vec_i_want.distance(position + field_position); // Calculate distance

                if (distance < 1.65)
                    && (flowerfields.flower_fields[which_field].flowers[what_flower].stage > 0)
                {
                    let tool = tool_q.single();
                    let mut how_much_pollen: i32 = 0;
                    let mut times_cause_color: i32 = 1;

                    match flowerfields.flower_fields[which_field].flowers[what_flower].flower_type {
                        Type::White => times_cause_color = tool.collect_amount_white as i32,
                        Type::Blue => times_cause_color = tool.collect_amount_blue as i32,
                        Type::Red => times_cause_color = tool.collect_amount_red as i32,
                    };

                    how_much_pollen = flowerfields.flower_fields[which_field].flowers[what_flower]
                        .how_many as i32
                        * tool.collect_amount as i32
                        * times_cause_color;

                    pollen_res.pollen_in_backpack += how_much_pollen as i64;

                    //      ==================
                    //      === Other Part ===
                    //      ==================

                    for (count, mut flower_transform) in flower_q.iter_mut().enumerate() {
                        if count as i32 == what_flower as i32 + which_flower_id_plus {
                            let flower_t = &mut *flower_transform; // Dereference Mut to get &mut Transform

                            flower_gets_harvested(
                                &mut flowerfields.flower_fields[which_field].flowers[what_flower],
                                flower_t,
                                &flower_que,
                            );

                            break;
                        }
                    }
                }
            }
        }
    }
}

fn flower_gets_harvested(
    flower: &mut Flower,
    flower_entity: &mut Transform,
    flower_q: &Query<&Flower, With<Flower>>,
) {
    flower.stage -= 1;
    //for flower_p in flower_q.iter() {
    //    println!("{:?}", flower_p);
    //}
    /*
    let mut y_pos: f32 = 0.0;

    match flower.stage as i32 {
        3 => y_pos = 0.0,
        2 => y_pos = -0.15,
        1 => y_pos = -0.3,
        0 => y_pos = -0.55,
        _ => println!("Shit"),
    };

    flower_entity.translation.y = y_pos;
    */
}

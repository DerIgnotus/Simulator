use crate::game::world::components::*;
use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub fn setup_world(
    mut commands: Commands,
    _materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    asset_server: Res<AssetServer>,
    mut flower_fields_res: ResMut<FlowerFields>,
) {
    let meshes: Vec<Handle<Scene>> = vec![
        asset_server.load("meshes/world.glb#Scene0"),
        asset_server.load("meshes/spawn.glb#Scene0"),
        asset_server.load("meshes/field_1.glb#Scene0"),
    ];
    let flower_meshes: Vec<Handle<Scene>> = vec![
        asset_server.load("meshes/flower_white_1.glb#Scene0"),
        asset_server.load("meshes/flower_white_2.glb#Scene0"),
        asset_server.load("meshes/flower_white_3.glb#Scene0"),
        asset_server.load("meshes/flower_blue_1.glb#Scene0"),
        asset_server.load("meshes/flower_blue_2.glb#Scene0"),
        asset_server.load("meshes/flower_blue_3.glb#Scene0"),
        asset_server.load("meshes/flower_red_1.glb#Scene0"),
        asset_server.load("meshes/flower_red_2.glb#Scene0"),
        asset_server.load("meshes/flower_red_3.glb#Scene0"),
    ];
    let flowers: Vec<Flower> = vec![
        Flower {
            flower_type: Type::White,
            stage: 3,
            how_many: 1,
        },
        Flower {
            flower_type: Type::White,
            stage: 3,
            how_many: 2,
        },
        Flower {
            flower_type: Type::White,
            stage: 3,
            how_many: 3,
        },
        Flower {
            flower_type: Type::Blue,
            stage: 3,
            how_many: 1,
        },
        Flower {
            flower_type: Type::Blue,
            stage: 3,
            how_many: 2,
        },
        Flower {
            flower_type: Type::Blue,
            stage: 3,
            how_many: 3,
        },
        Flower {
            flower_type: Type::Red,
            stage: 3,
            how_many: 1,
        },
        Flower {
            flower_type: Type::Red,
            stage: 3,
            how_many: 2,
        },
        Flower {
            flower_type: Type::Red,
            stage: 3,
            how_many: 3,
        },
    ];

    let mut flower_field_1 = FlowerField {
        which_field: "field_1".to_string(),
        field_pos: vec3(0.0, 0.01, 15.0),
        positions: Vec::new(),
        flowers: Vec::new(),
    };

    let mut flower_field_2 = FlowerField {
        which_field: "field_2".to_string(),
        field_pos: vec3(0.0, 0.01, -15.0),
        positions: Vec::new(),
        flowers: Vec::new(),
    };

    let ground = (
        SceneBundle {
            scene: meshes[0].clone(),
            ..Default::default()
        },
        Collider::cuboid(200.0, 0.01, 200.0),
        RigidBody::Fixed,
        Ground {},
    );

    let spawn = (
        SceneBundle {
            scene: meshes[1].clone(),
            ..Default::default()
        },
        Collider::cuboid(2.0, 0.2, 2.0),
        RigidBody::Fixed,
        Ground {},
    );

    commands
        .spawn((
            SceneBundle {
                scene: meshes[2].clone(),
                transform: Transform::from_xyz(0.0, 0.01, 15.0),
                ..Default::default()
            },
            Collider::cuboid(10.0, 0.1, 7.0),
            RigidBody::Fixed,
            Ground {},
            flower_field_1.clone(),
        ))
        .with_children(|parent| {
            for row_flowers in 0..FIELD_1_FLOWERS_ROW {
                for column_flowers in 0..FIELD_1_FLOWERS_COLUMN {
                    let mut rng = rand::thread_rng();
                    let white_blue_red = rng.gen_range(0..=99);
                    let one_two_three = rng.gen_range(0..=99);

                    let mut x_value: f32 = row_flowers as f32;
                    let mut y_value: f32 = column_flowers as f32;

                    x_value -= 9.5;
                    y_value -= 6.0;

                    if white_blue_red < 70 {
                        if one_two_three < 70 {
                            flower_field_1.flowers.push(flowers[0].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[0].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[0].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_1.flowers.push(flowers[1].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[1].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[1].clone(),
                            ));
                        } else {
                            flower_field_1.flowers.push(flowers[2].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[2].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[2].clone(),
                            ));
                        }
                    } else if white_blue_red < 85 {
                        if one_two_three < 70 {
                            flower_field_1.flowers.push(flowers[3].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[3].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[3].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_1.flowers.push(flowers[4].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[4].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[4].clone(),
                            ));
                        } else {
                            flower_field_1.flowers.push(flowers[5].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[5].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[5].clone(),
                            ));
                        }
                    } else if white_blue_red <= 100 {
                        if one_two_three < 70 {
                            flower_field_1.flowers.push(flowers[6].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[6].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[6].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_1.flowers.push(flowers[7].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[7].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[7].clone(),
                            ));
                        } else {
                            flower_field_1.flowers.push(flowers[8].clone());
                            flower_field_1.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[8].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[8].clone(),
                            ));
                        }
                    }
                }
            }
        });

    commands
        .spawn((
            SceneBundle {
                scene: meshes[2].clone(),
                transform: Transform::from_xyz(0.0, 0.01, -15.0),
                ..Default::default()
            },
            Collider::cuboid(10.0, 0.1, 7.0),
            RigidBody::Fixed,
            Ground {},
            flower_field_2.clone(),
        ))
        .with_children(|parent| {
            for row_flowers in 0..FIELD_2_FLOWERS_ROW {
                for column_flowers in 0..FIELD_2_FLOWERS_COLUMN {
                    let mut rng = rand::thread_rng();
                    let white_blue_red = rng.gen_range(0..=99);
                    let one_two_three = rng.gen_range(0..=99);

                    let mut x_value: f32 = row_flowers as f32;
                    let mut y_value: f32 = column_flowers as f32;

                    x_value -= 9.5;
                    y_value -= 6.0;

                    if white_blue_red < 70 {
                        if one_two_three < 70 {
                            flower_field_2.flowers.push(flowers[0].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[0].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[0].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_2.flowers.push(flowers[1].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[1].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[1].clone(),
                            ));
                        } else {
                            flower_field_2.flowers.push(flowers[2].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[2].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[2].clone(),
                            ));
                        }
                    } else if white_blue_red < 85 {
                        if one_two_three < 70 {
                            flower_field_2.flowers.push(flowers[3].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[3].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[3].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_2.flowers.push(flowers[4].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[4].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[4].clone(),
                            ));
                        } else {
                            flower_field_2.flowers.push(flowers[5].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[5].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[5].clone(),
                            ));
                        }
                    } else if white_blue_red <= 100 {
                        if one_two_three < 70 {
                            flower_field_2.flowers.push(flowers[6].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[6].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[6].clone(),
                            ));
                        } else if one_two_three < 90 {
                            flower_field_2.flowers.push(flowers[7].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[7].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[7].clone(),
                            ));
                        } else {
                            flower_field_2.flowers.push(flowers[8].clone());
                            flower_field_2.positions.push(Vec3 {
                                x: x_value,
                                y: 0.0,
                                z: y_value,
                            });

                            parent.spawn((
                                SceneBundle {
                                    scene: flower_meshes[8].clone(),
                                    transform: Transform::from_xyz(x_value, 0.0, y_value),
                                    ..Default::default()
                                },
                                flowers[8].clone(),
                            ));
                        }
                    }
                }
            }
        });

    flower_fields_res.flower_fields.push(flower_field_1.clone());
    flower_fields_res.flower_fields.push(flower_field_2.clone());

    commands.spawn(ground);
    commands.spawn(spawn);

    ambient_light.brightness = 0.8;
}
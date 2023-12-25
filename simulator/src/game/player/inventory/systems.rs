use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_INVENTORY: Mutex<Inventory> = Mutex::new(Inventory::default());
}

use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use super::components::*;

pub fn inventory_hotbar_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let default_slot = Slot {
        item: Item {
            name: "No Item".to_string(),
            count: 0,
            description: "No Item Desciption".to_string(),
            image: "sprites/slot_hotbar.png".to_string(),
        },
        has_item: false,
    };

    let mut slot_pos: f32 = 600.0;

    for _ in 0..HOTBAR_SLOT_COUNT {
        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(slot_pos),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                image: asset_server.load(default_slot.item.image.clone()).into(),
                ..Default::default()
            },
            default_slot.clone(),
        ));

        slot_pos += 125.0;
    }
}

pub fn update_hotbar() {}

pub fn inventory_setup(mut commands: Commands) {
    // List with hidden overflow
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.),
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.9).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(100.0),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::rgba(0.10, 0.10, 0.10, 0.5).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .insert(ParentInv {});
                        });
                });
        })
        .insert(InventoryHide {})
        .insert(Visibility::Visible);
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub fn display_inventory_items(
    mut commands: Commands,
    mut displayed_items: Local<HashSet<String>>,
    mut count_text_entities: Local<HashMap<String, Entity>>, // Keep track of count text entities
    inventory_query: Query<Entity, With<ParentInv>>,
    mut text_query: Query<&mut Text, With<CountText>>,
) {
    if let Ok(global_inventory) = GLOBAL_INVENTORY.lock() {
        for parent_entity in inventory_query.iter() {
            for item in &global_inventory.items {
                let print = format!("{}: {}", item.name, item.count);

                // Update count text entity if the count has changed
                if let Some(&count_entity) = count_text_entities.get(&item.name) {
                    if let Ok(mut text) = text_query.get_mut(count_entity) {
                        text.sections[0].value = print.clone();
                    }
                } else {
                    let description = item.description.clone();

                    commands
                        .entity(parent_entity)
                        .with_children(|parent_spawn| {
                            // Display item name and count
                            let count_entity = parent_spawn
                                .spawn(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: print.clone(),
                                            style: TextStyle {
                                                font_size: 35.0,
                                                // Add other text styling if needed
                                                ..Default::default()
                                            },
                                        }],
                                        // Add other text properties if needed
                                        ..Default::default()
                                    },
                                    // Add other bundle properties if needed
                                    ..Default::default()
                                })
                                .insert(CountText {})
                                .id();

                            // Add the displayed count text entity to the HashMap
                            count_text_entities.insert(item.name.clone(), count_entity);
                            displayed_items.insert(item.name.clone());

                            // Display item description
                            parent_spawn.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: description,
                                        style: TextStyle {
                                            font_size: 20.0,
                                            // Add other text styling if needed
                                            ..Default::default()
                                        },
                                    }],
                                    // Add other text properties if needed
                                    ..Default::default()
                                },
                                // Add other bundle properties if needed
                                ..Default::default()
                            });
                        });
                }
            }
        }
    }
}

pub fn toggle_inventory(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<InventoryHide>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        let mut visible = query.single_mut();

        if *visible == Visibility::Visible {
            *visible = Visibility::Hidden;
        } else {
            *visible = Visibility::Visible;
        }
    }
}

use bevy::prelude::*;

use crate::{
    spider::{Spider, SpiderAppearance, SpiderShapeType},
    terrain::{CurrentTerrain, TerrainChangeEvent, TerrainType},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    toggle_menu,
                    handle_terrain_buttons,
                    handle_color_buttons,
                    handle_shape_buttons,
                    update_ui_visibility,
                    update_button_highlights,
                ),
            );
    }
}

#[derive(Resource)]
struct MenuState {
    visible: bool,
}

impl Default for MenuState {
    fn default() -> Self {
        Self { visible: false }
    }
}

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct TerrainButton {
    terrain_type: TerrainType,
}

#[derive(Component)]
struct BodyColorButton {
    color: Color,
}

#[derive(Component)]
struct LegColorButton {
    color: Color,
}

#[derive(Component)]
struct ShapeButton {
    shape: SpiderShapeType,
}

#[derive(Component)]
struct MenuHintText;

const BUTTON_SIZE: Val = Val::Px(40.0);
const BUTTON_MARGIN: UiRect = UiRect::all(Val::Px(3.0));

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        MenuHintText,
        Text::new("Press [TAB] for menu"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    commands
        .spawn((
            MenuRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(35.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            BorderRadius::all(Val::Px(8.0)),
            Visibility::Hidden,
        ))
        .with_children(|menu| {
            menu.spawn((
                Text::new("Terrain"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                for terrain_type in TerrainType::all() {
                    spawn_text_button(
                        row,
                        terrain_type.name(),
                        TerrainButton {
                            terrain_type: *terrain_type,
                        },
                    );
                }
            });

            menu.spawn((
                Text::new("Body Color"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                let colors = [
                    ("Black", Color::BLACK),
                    ("Red", Color::srgb(0.8, 0.1, 0.1)),
                    ("Green", Color::srgb(0.1, 0.6, 0.1)),
                    ("Blue", Color::srgb(0.1, 0.2, 0.8)),
                    ("Purple", Color::srgb(0.5, 0.0, 0.7)),
                    ("Orange", Color::srgb(0.9, 0.5, 0.0)),
                    ("Brown", Color::srgb(0.4, 0.25, 0.1)),
                    ("White", Color::srgb(0.9, 0.9, 0.9)),
                ];

                for (name, color) in colors {
                    spawn_color_swatch(row, name, color, BodyColorButton { color });
                }
            });

            menu.spawn((
                Text::new("Leg Color"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                let colors = [
                    ("Dark Gray", Color::srgb(0.25, 0.25, 0.25)),
                    ("Red", Color::srgb(0.6, 0.1, 0.1)),
                    ("Green", Color::srgb(0.1, 0.5, 0.1)),
                    ("Blue", Color::srgb(0.1, 0.15, 0.6)),
                    ("Purple", Color::srgb(0.4, 0.0, 0.5)),
                    ("Yellow", Color::srgb(0.7, 0.7, 0.0)),
                    ("Brown", Color::srgb(0.3, 0.2, 0.08)),
                    ("White", Color::srgb(0.85, 0.85, 0.85)),
                ];

                for (name, color) in colors {
                    spawn_color_swatch(row, name, color, LegColorButton { color });
                }
            });

            menu.spawn((
                Text::new("Body Shape"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                let shapes = [
                    ("Box", SpiderShapeType::Box),
                    ("Sphere", SpiderShapeType::Sphere),
                    ("Flat", SpiderShapeType::Flat),
                    ("Long", SpiderShapeType::Long),
                ];

                for (name, shape) in shapes {
                    spawn_text_button(row, name, ShapeButton { shape });
                }
            });
        });
}

fn spawn_text_button(parent: &mut ChildSpawnerCommands, text: &str, marker: impl Component) {
    parent
        .spawn((
            marker,
            Button,
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            BorderRadius::all(Val::Px(4.0)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_color_swatch(
    parent: &mut ChildSpawnerCommands,
    _name: &str,
    color: Color,
    marker: impl Component,
) {
    parent.spawn((
        marker,
        Button,
        Node {
            width: BUTTON_SIZE,
            height: BUTTON_SIZE,
            border: UiRect::all(Val::Px(2.0)),
            margin: BUTTON_MARGIN,
            ..default()
        },
        BackgroundColor(color),
        BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        BorderRadius::all(Val::Px(4.0)),
    ));
}

fn toggle_menu(input: Res<ButtonInput<KeyCode>>, mut menu_state: ResMut<MenuState>) {
    if input.just_pressed(KeyCode::Tab) {
        menu_state.visible = !menu_state.visible;
    }
}

fn update_ui_visibility(
    menu_state: Res<MenuState>,
    mut menu_root: Query<&mut Visibility, With<MenuRoot>>,
) {
    if let Ok(mut vis) = menu_root.single_mut() {
        *vis = if menu_state.visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_terrain_buttons(
    interactions: Query<(&Interaction, &TerrainButton), Changed<Interaction>>,
    mut terrain_events: EventWriter<TerrainChangeEvent>,
) {
    for (interaction, terrain_btn) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            terrain_events.write(TerrainChangeEvent {
                new_terrain: terrain_btn.terrain_type,
            });
        }
    }
}

fn handle_color_buttons(
    body_interactions: Query<(&Interaction, &BodyColorButton), Changed<Interaction>>,
    leg_interactions: Query<(&Interaction, &LegColorButton), Changed<Interaction>>,
    mut spider: Query<&mut SpiderAppearance, With<Spider>>,
) {
    for (interaction, btn) in body_interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut appearance) = spider.single_mut() {
                appearance.color_config.body_color = btn.color;
                appearance.dirty = true;
            }
        }
    }

    for (interaction, btn) in leg_interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut appearance) = spider.single_mut() {
                appearance.color_config.leg_color = btn.color;
                appearance.dirty = true;
            }
        }
    }
}

fn handle_shape_buttons(
    interactions: Query<(&Interaction, &ShapeButton), Changed<Interaction>>,
    mut spider: Query<&mut SpiderAppearance, With<Spider>>,
) {
    for (interaction, btn) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut appearance) = spider.single_mut() {
                appearance.shape = btn.shape;
                appearance.dirty = true;
            }
        }
    }
}

fn update_button_highlights(
    mut terrain_buttons: Query<(&TerrainButton, &mut BorderColor)>,
    mut shape_buttons: Query<(&ShapeButton, &mut BorderColor), Without<TerrainButton>>,
    current_terrain: Res<CurrentTerrain>,
    spider: Query<&SpiderAppearance, With<Spider>>,
) {
    let active_border = BorderColor(Color::srgb(1.0, 1.0, 0.0));
    let inactive_border = BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.15));

    for (btn, mut border) in terrain_buttons.iter_mut() {
        *border = if btn.terrain_type == current_terrain.terrain_type {
            active_border
        } else {
            inactive_border
        };
    }

    if let Ok(appearance) = spider.single() {
        for (btn, mut border) in shape_buttons.iter_mut() {
            *border = if btn.shape == appearance.shape {
                active_border
            } else {
                inactive_border
            };
        }
    }
}

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
                    update_ui_visibility,
                    handle_terrain_buttons,
                    handle_body_color_buttons,
                    handle_leg_color_buttons,
                    handle_shape_buttons,
                    update_terrain_highlights,
                    update_shape_highlights,
                    update_body_color_highlights,
                    update_leg_color_highlights,
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

const SWATCH_SIZE: Val = Val::Px(36.0);

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        MenuHintText,
        Text::new("Press [TAB] for menu | RMB drag to orbit | Scroll to zoom | Q/E rotate | R/F tilt"),
        TextFont {
            font_size: 16.0,
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
                top: Val::Px(40.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(8.0),
                min_width: Val::Px(300.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
            BorderRadius::all(Val::Px(8.0)),
            Visibility::Hidden,
        ))
        .with_children(|menu| {
            spawn_section_label(menu, "Terrain");
            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(4.0),
                row_gap: Val::Px(4.0),
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

            // === BODY COLOR ===
            spawn_section_label(menu, "Body Color");
            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(4.0),
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|row| {
                let colors = [
                    Color::BLACK,
                    Color::srgb(0.8, 0.1, 0.1),
                    Color::srgb(0.1, 0.6, 0.1),
                    Color::srgb(0.1, 0.2, 0.8),
                    Color::srgb(0.5, 0.0, 0.7),
                    Color::srgb(0.9, 0.5, 0.0),
                    Color::srgb(0.4, 0.25, 0.1),
                    Color::srgb(0.9, 0.9, 0.9),
                ];
                for color in colors {
                    spawn_color_swatch(row, color, BodyColorButton { color });
                }
            });

            spawn_section_label(menu, "Leg Color");
            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(4.0),
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|row| {
                let colors = [
                    Color::srgb(0.25, 0.25, 0.25),
                    Color::srgb(0.6, 0.1, 0.1),
                    Color::srgb(0.1, 0.5, 0.1),
                    Color::srgb(0.1, 0.15, 0.6),
                    Color::srgb(0.4, 0.0, 0.5),
                    Color::srgb(0.7, 0.7, 0.0),
                    Color::srgb(0.3, 0.2, 0.08),
                    Color::srgb(0.85, 0.85, 0.85),
                ];
                for color in colors {
                    spawn_color_swatch(row, color, LegColorButton { color });
                }
            });

            spawn_section_label(menu, "Body Shape");
            menu.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(4.0),
                row_gap: Val::Px(4.0),
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

fn spawn_section_label(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text.to_string()),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
    ));
}

fn spawn_text_button(parent: &mut ChildSpawnerCommands, text: &str, marker: impl Component) {
    parent
        .spawn((
            marker,
            Button,
            Interaction::None,
            Node {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
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
    color: Color,
    marker: impl Component,
) {
    parent.spawn((
        marker,
        Button,
        Interaction::None,
        Node {
            width: SWATCH_SIZE,
            height: SWATCH_SIZE,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(color),
        BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
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
    for (interaction, btn) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            terrain_events.send(TerrainChangeEvent {
                new_terrain: btn.terrain_type,
            });
        }
    }
}

fn handle_body_color_buttons(
    interactions: Query<(&Interaction, &BodyColorButton), Changed<Interaction>>,
    mut spider: Query<&mut SpiderAppearance, With<Spider>>,
) {
    for (interaction, btn) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut appearance) = spider.single_mut() {
                appearance.color_config.body_color = btn.color;
                appearance.dirty = true;
            }
        }
    }
}

fn handle_leg_color_buttons(
    interactions: Query<(&Interaction, &LegColorButton), Changed<Interaction>>,
    mut spider: Query<&mut SpiderAppearance, With<Spider>>,
) {
    for (interaction, btn) in interactions.iter() {
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

fn update_terrain_highlights(
    mut buttons: Query<(&TerrainButton, &mut BorderColor)>,
    current_terrain: Res<CurrentTerrain>,
) {
    for (btn, mut border) in buttons.iter_mut() {
        *border = if btn.terrain_type == current_terrain.terrain_type {
            BorderColor(Color::srgb(1.0, 1.0, 0.0))
        } else {
            BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.15))
        };
    }
}

fn update_shape_highlights(
    mut buttons: Query<(&ShapeButton, &mut BorderColor)>,
    spider: Query<&SpiderAppearance, With<Spider>>,
) {
    if let Ok(appearance) = spider.single() {
        for (btn, mut border) in buttons.iter_mut() {
            *border = if btn.shape == appearance.shape {
                BorderColor(Color::srgb(1.0, 1.0, 0.0))
            } else {
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.15))
            };
        }
    }
}

fn update_body_color_highlights(
    mut buttons: Query<(&BodyColorButton, &mut BorderColor)>,
    spider: Query<&SpiderAppearance, With<Spider>>,
) {
    if let Ok(appearance) = spider.single() {
        for (btn, mut border) in buttons.iter_mut() {
            *border = if colors_match(btn.color, appearance.color_config.body_color) {
                BorderColor(Color::srgb(1.0, 1.0, 0.0))
            } else {
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2))
            };
        }
    }
}

fn update_leg_color_highlights(
    mut buttons: Query<(&LegColorButton, &mut BorderColor)>,
    spider: Query<&SpiderAppearance, With<Spider>>,
) {
    if let Ok(appearance) = spider.single() {
        for (btn, mut border) in buttons.iter_mut() {
            *border = if colors_match(btn.color, appearance.color_config.leg_color) {
                BorderColor(Color::srgb(1.0, 1.0, 0.0))
            } else {
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2))
            };
        }
    }
}

fn colors_match(a: Color, b: Color) -> bool {
    let a = a.to_srgba();
    let b = b.to_srgba();
    (a.red - b.red).abs() < 0.01
        && (a.green - b.green).abs() < 0.01
        && (a.blue - b.blue).abs() < 0.01
}
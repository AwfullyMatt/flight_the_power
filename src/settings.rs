use bevy::{prelude::*, scene::ron::de::from_reader};
use serde::{Deserialize, Serialize};
use std::fs::File;

use crate::AppState;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), startup)
            .add_systems(OnExit(AppState::Settings), cleanup)
            .add_systems(
                Update,
                settings_button_interaction.run_if(in_state(AppState::Settings)),
            );

        app.insert_resource(Settings::load());
    }
}

#[derive(Default, Deserialize, Serialize, Resource)]
pub struct Settings {
    pub resolution: Resolution,
    pub monitor: usize,
}
impl Settings {
    fn load() -> Self {
        let input_path = format!("{}/ron/settings.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(input_path.clone()).expect("Failed opening file");
        let settings: Settings = match from_reader(f) {
            Ok(x) => {
                info!("[INITIALIZED] Settings");
                x
            }
            Err(e) => {
                eprintln!("[ERROR] Could not deserialize {}. \n{}", input_path, e);
                Self::default()
            }
        };
        settings
    }
}

#[allow(dead_code)]
#[derive(Default, Deserialize, Serialize)]
pub enum Resolution {
    #[default]
    SD, // 480p
    HD,  // 1080p
    UHD, // 2160p
}
impl Resolution {
    pub fn vec2(&self) -> Vec2 {
        use Resolution::*;

        match self {
            SD => Vec2 { x: 640., y: 480. },
            HD => Vec2 { x: 1920., y: 1080. },
            UHD => Vec2 { x: 3840., y: 2160. },
        }
    }

    pub fn scale(&self) -> f32 {
        use Resolution::*;

        match self {
            SD => 2.0,
            HD => 5.0,
            UHD => 10.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub enum Pallette {
    White,
    Lighter,
    Light,
    Dark,
    Darker,
    Black,
}
impl Pallette {
    pub fn srgb(&self) -> Color {
        use Pallette::*;
        match self {
            White => Color::srgb(1., 1., 1.),
            Lighter => Color::srgb(0.8275, 0.8275, 0.8275),
            Light => Color::srgb(0.06549, 0.06549, 0.06549),
            Dark => Color::srgb(0.3647, 0.3647, 0.3647),
            Darker => Color::srgb(0.2118, 0.2118, 0.2118),
            Black => Color::srgb(0., 0., 0.),
        }
    }
}

#[derive(Component)]
struct CleanupSettingsMenu;

#[derive(Component)]
pub enum SettingsMenuButton {
    SD,
    HD,
    UHD,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PublicPixel.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            CleanupSettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: Val::Px(320.0),
                    height: Val::Px(115.0),
                    border: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_child((
                    Text::new("RESOLUTION"),
                    TextFont {
                        font: font.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb()),
                    SettingsMenuButton::SD,
                ))
                .with_child((
                    Text::new("SD"),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb()),
                    SettingsMenuButton::HD,
                ))
                .with_child((
                    Text::new("HD"),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb()),
                    SettingsMenuButton::UHD,
                ))
                .with_child((
                    Text::new("UHD"),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        });
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupSettingsMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANED] Settings Menu.");
    }
}

fn settings_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &SettingsMenuButton,
        ),
        (Changed<Interaction>, With<SettingsMenuButton>),
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, mut border_color, children, _smb) in
        &mut interaction_query
    {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                background_color.0 = Pallette::Lighter.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
            Interaction::Hovered => {
                background_color.0 = Pallette::Darker.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
            Interaction::Pressed => {
                background_color.0 = Pallette::Darker.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
        }
    }
}

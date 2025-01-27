use bevy::{
    prelude::*,
    scene::ron::{de::from_reader, ser::to_writer},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, ErrorKind, Result},
};

use crate::{
    save::Saveable,
    ui::{Pallette, UIButton},
    AppState,
};

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
                (settings_button_interaction, escape_to_menu).run_if(in_state(AppState::Settings)),
            )
            .add_systems(Update, update_settings);

        app.insert_resource(Settings::load("settings.ron").unwrap_or_default());
    }
}

#[derive(Default, Deserialize, Serialize, Resource)]
pub struct Settings {
    resolution: Resolution,
    monitor: usize,
}
impl Settings {
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }

    /* pub fn set_monitor(&mut self, u: usize) {
        self.monitor = u;
    } */

    pub fn resolution(&self) -> Vec2 {
        self.resolution.vec2()
    }

    pub fn sprite_scale(&self) -> f32 {
        self.resolution.scale()
    }

    pub fn monitor_index(&self) -> usize {
        self.monitor
    }
}
impl Saveable for Settings {
    fn save(&self, filename: &str) -> Result<()> {
        let path = format!("{}/ron/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let file = File::create(path)?;
        to_writer(file, self).map_err(|e| Error::new(ErrorKind::Other, e))
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = format!("{}/ron/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let file = File::open(path)?;
        from_reader(file).map_err(|e| Error::new(ErrorKind::Other, e))
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
    fn vec2(&self) -> Vec2 {
        use Resolution::*;

        match self {
            SD => Vec2 { x: 640., y: 480. },
            HD => Vec2 { x: 1920., y: 1080. },
            UHD => Vec2 { x: 3840., y: 2160. },
        }
    }

    fn scale(&self) -> f32 {
        use Resolution::*;

        match self {
            SD => 2.0,
            HD => 5.0,
            UHD => 10.0,
        }
    }
}

#[derive(Component)]
struct CleanupSettingsMenu;

#[derive(Component, Debug)]
pub enum SettingsMenuButton {
    SD,
    HD,
    UHD,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Programmatic Button Size
    let font = asset_server.load("fonts/PublicPixel.ttf");
    let parent_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceEvenly,
        flex_direction: FlexDirection::Row,
        ..default()
    };

    let child_node = Node {
        width: Val::Px(320.0),
        height: Val::Px(115.0),
        border: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let style = (
        BorderColor(Pallette::Black.srgb()),
        BorderRadius::all(Val::Percent(10.0)),
        BackgroundColor(Pallette::Lighter.srgb().into()),
    );

    commands
        .spawn((parent_node, CleanupSettingsMenu))
        .with_children(|parent| {
            parent.spawn(child_node.clone()).with_child((
                Text::new("RESOLUTION"),
                TextFont {
                    font: font.clone(),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Pallette::Black.srgb()),
            ));

            for i in 0..3 {
                let text: Text = match i {
                    0 => Text::new("SD"),
                    1 => Text::new("HD"),
                    _ => Text::new("UHD"),
                };
                let smb: SettingsMenuButton = match i {
                    0 => SettingsMenuButton::SD,
                    1 => SettingsMenuButton::HD,
                    _ => SettingsMenuButton::UHD,
                };

                parent
                    .spawn((child_node.clone(), Button, smb, UIButton, style.clone()))
                    .with_child((
                        text,
                        TextFont {
                            font: font.clone(),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Pallette::Black.srgb()),
                    ));
            }
        });
    info!("[SPAWNED] Settings Menu Entities.");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupSettingsMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANED] Settings Menu.");
    }
}

fn settings_button_interaction(
    mut settings: ResMut<Settings>,
    mut interaction_query: Query<
        (&Interaction, &SettingsMenuButton),
        (Changed<Interaction>, With<SettingsMenuButton>),
    >,
) {
    use SettingsMenuButton::*;

    for (interaction, smb) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => match smb {
                SD => {
                    settings.set_resolution(Resolution::SD);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
                HD => {
                    settings.set_resolution(Resolution::HD);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
                UHD => {
                    settings.set_resolution(Resolution::UHD);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
            },
            _ => {}
        }
    }
}

fn update_settings(settings: Res<Settings>, mut query_window: Query<&mut Window>) {
    if settings.is_changed() {
        if let Ok(mut window) = query_window.get_single_mut() {
            // SET WINDOW RESOLUTION
            window
                .resolution
                .set(settings.resolution().x, settings.resolution().y);
            // SET MONITOR SELECTION
            window
                .position
                .center(MonitorSelection::Index(settings.monitor_index()));
            info!(
                "[INITIALIZED] Window Resolution : ({},{})",
                settings.resolution().x,
                settings.resolution().y
            );
        }
    }
}

fn escape_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            AppState::Settings => {
                next_state.set(AppState::Menu);
                info!("[MODIFIED] Appstate >> Settings");
            }
            _ => {}
        }
    }
}

// TODO: Physical Back Button, Monitor Selection Functionality

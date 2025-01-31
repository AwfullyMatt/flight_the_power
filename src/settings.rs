use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::{
    save::{format_load, format_save, Saveable},
    ui::{Pallette, UIButton, UIButtonChildNode, UIButtonParentNode},
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

        app.insert_resource(
            Settings::load("settings.ron").expect("[ERROR] Could not load settings.ron"),
        );
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
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}

#[allow(dead_code)]
#[derive(Default, Deserialize, Serialize)]
pub enum Resolution {
    #[default]
    Sd, // 480p
    Hd,  // 1080p
    Uhd, // 2160p
}
impl Resolution {
    fn vec2(&self) -> Vec2 {
        use Resolution::*;

        match self {
            Sd => Vec2 { x: 640., y: 480. },
            Hd => Vec2 { x: 1920., y: 1080. },
            Uhd => Vec2 { x: 3840., y: 2160. },
        }
    }

    fn scale(&self) -> f32 {
        use Resolution::*;

        match self {
            Sd => 2.0,
            Hd => 5.0,
            Uhd => 10.0,
        }
    }
}

#[derive(Component)]
struct CleanupSettingsMenu;

#[derive(Component, Debug)]
pub enum SettingsMenuButton {
    Sd,
    Hd,
    Uhd,
    Back,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Programmatic Button Size
    let font = asset_server.load("fonts/PublicPixel.ttf");

    let style = (
        BorderColor(Pallette::Black.srgb()),
        BorderRadius::all(Val::Percent(10.0)),
        BackgroundColor(Pallette::Lighter.srgb()),
    );

    // SPAWN RESOLUTION SETTINGS NODE
    commands
        .spawn((
            UIButtonParentNode::new(100.0, 33.0, 0.0),
            UIButtonParentNode::marker(),
            CleanupSettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((UIButtonChildNode::default(), UIButtonChildNode::marker()))
                .with_child((
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
                    0 => SettingsMenuButton::Sd,
                    1 => SettingsMenuButton::Hd,
                    _ => SettingsMenuButton::Uhd,
                };

                parent
                    .spawn((
                        UIButtonChildNode::default(),
                        UIButtonChildNode::marker(),
                        Button,
                        smb,
                        UIButton,
                        style,
                    ))
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

    // SPAWN BACK BUTTON NODE
    commands
        .spawn((
            UIButtonParentNode::new(100.0, 33.0, 70.0),
            UIButtonParentNode::marker(),
            CleanupSettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    UIButtonChildNode::default(),
                    UIButtonChildNode::marker(),
                    Button,
                    SettingsMenuButton::Back,
                    UIButton,
                    style,
                ))
                .with_child((
                    Text::new("BACK"),
                    TextFont {
                        font: font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        });

    info!("[SPAWNED] Settings Menu Entities.");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupSettingsMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Settings Menu Entities");
    }
}

fn settings_button_interaction(
    mut settings: ResMut<Settings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &SettingsMenuButton),
        (Changed<Interaction>, With<SettingsMenuButton>),
    >,
) {
    use SettingsMenuButton::*;

    for (interaction, smb) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match smb {
                Sd => {
                    settings.set_resolution(Resolution::Sd);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
                Hd => {
                    settings.set_resolution(Resolution::Hd);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
                Uhd => {
                    settings.set_resolution(Resolution::Uhd);
                    info!("[MODIFIED] Resolution - {smb:?}");
                }
                Back => {
                    next_state.set(AppState::Menu);
                    info!("[MODIFIED] AppState - Menu");
                }
            }
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
    if keys.just_pressed(KeyCode::Escape) && *current_state.get() == AppState::Settings {
        next_state.set(AppState::Menu);
        info!("[MODIFIED] Appstate >> Settings");
    }
}

// TODO: Physical Back Button, Monitor Selection Functionality

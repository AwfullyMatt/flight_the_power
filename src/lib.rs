mod loading;
mod menu;
mod settings;

use bevy::prelude::*;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use settings::{Pallette, Settings, SettingsPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn name(&self) -> &str {
        "Game Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Flight the Power".to_string(),
                        canvas: Some("#FTP".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        window_theme: Some(bevy::window::WindowTheme::Dark),
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                // PIXEL PERFECT
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins((LoadingPlugin, MenuPlugin, SettingsPlugin));

        app.add_sub_state::<GameState>();

        app.add_systems(Startup, startup);

        app.init_state::<AppState>();

        app.insert_resource(ClearColor(Pallette::Dark.srgb()));
    }
}

fn startup(mut commands: Commands, mut query_window: Query<&mut Window>, settings: Res<Settings>) {
    // SPAWN CAMERA2D
    commands.spawn(Camera2d);

    // SET WINDOW RESOLUTION ACCORDING TO SAVED SETTING
    if let Ok(mut window) = query_window.get_single_mut() {
        // SET WINDOW RESOLUTION
        window
            .resolution
            .set(settings.resolution.vec2().x, settings.resolution.vec2().y);
        // SET MONITOR SELECTION
        window
            .position
            .center(MonitorSelection::Index(settings.monitor));
        info!(
            "[INITIALIZED] Window Resolution : ({},{})",
            settings.resolution.vec2().x,
            settings.resolution.vec2().y
        );
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Playing,
    Settings,
    Exit,
}

#[derive(SubStates, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[source(AppState = AppState::Playing)]
pub enum GameState {
    #[default]
    Home,
}

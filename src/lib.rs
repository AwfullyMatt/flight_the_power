mod game;
mod loading;
mod menu;
mod save;
mod settings;
mod ui;

use std::{io::Cursor, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use game::GameLoopPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use save::SavePlugin;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsPlugin};
use ui::{Pallette, UIPlugin};
use winit::window::Icon;

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
                        prevent_default_event_handling: false,
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
        app.add_plugins((
            GameLoopPlugin,
            LoadingPlugin,
            MenuPlugin,
            SavePlugin,
            SettingsPlugin,
            UIPlugin,
        ));

        app.add_sub_state::<PauseState>();

        app.add_systems(Startup, startup);

        app.init_state::<AppState>();

        app.init_state::<PauseState>();

        app.insert_resource(ClearColor(Pallette::Dark.srgb()));
    }
}

fn startup(
    mut commands: Commands,
    mut query_window: Query<&mut Window>,
    settings: Res<Settings>,
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    // SPAWN CAMERA2D
    commands.spawn(Camera2d);

    // SET WINDOW RESOLUTION ACCORDING TO SAVED SETTING
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

    // SET WINDOW ICON
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
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
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct Title(String);
impl Title {
    pub fn title(&self) -> &String {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct ID(usize);
impl ID {
    pub fn id(&self) -> &usize {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct Cost(i64);
impl Cost {
    pub fn cost(&self) -> &i64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct ProdAmount(i64);
impl Cost {
    pub fn prod_amt(&self) -> &i64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct ProdRate(f64);
impl ProdRate {
    pub fn prod_rate(&self) -> &f64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct MaxOwned(i64);
impl MaxOwned {
    pub fn max_owned(&self) -> &i64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct CurrentOwned(i64);
impl CurrentOwned {
    pub fn current_owned(&self) -> &i64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct UnlockBound(i64);
impl UnlockBound {
    pub fn unlock_bound(&self) -> &i64 {
        &self.0
    }
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct ProdTimer(Timer);
impl ProdTimer {
    pub fn new(secs: f64, mode: TimerMode) -> Self {
        Self(Timer::new(Duration::from_secs_f64(secs), mode))
    }
}

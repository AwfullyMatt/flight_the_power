use bevy::{prelude::*, scene::ron::de::from_reader};
use serde::{Deserialize, Serialize};
use std::fs::File;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);

        app.insert_resource(Settings::load());
    }
}

fn startup() {}

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
    WHITE,
    LIGHTER,
    LIGHT,
    DARK,
    DARKER,
    BLACK,
}
impl Pallette {
    pub fn srgb(&self) -> Color {
        use Pallette::*;
        match self {
            WHITE => Color::srgb(1., 1., 1.),
            LIGHTER => Color::srgb(0.8275, 0.8275, 0.8275),
            LIGHT => Color::srgb(0.06549, 0.06549, 0.06549),
            DARK => Color::srgb(0.3647, 0.3647, 0.3647),
            DARKER => Color::srgb(0.2118, 0.2118, 0.2118),
            BLACK => Color::srgb(0., 0., 0.),
        }
    }
}

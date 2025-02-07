#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::{App, AppExit};
use flight_the_power::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}

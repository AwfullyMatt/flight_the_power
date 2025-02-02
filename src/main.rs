#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
use bevy::app::{App, AppExit};
use flightthepower::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}

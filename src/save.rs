use bevy::prelude::*;
use std::io::Result;

use crate::{settings::Settings, AppState};

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn name(&self) -> &str {
        "Save Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<Save>()
            .add_systems(OnEnter(AppState::Exit), evw_save)
            .add_systems(Update, evr_save);
    }
}

pub trait Saveable {
    fn save(&self, filename: &str) -> Result<()>;
    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Event)]
pub struct Save;

fn evr_save(mut evr_save: EventReader<Save>, settings: Res<Settings>) {
    for _ev in evr_save.read() {
        info!("[EVENT] [READ] Save Game");
        let _ = settings.save("settings.ron");
    }
}

fn evw_save(mut evw_save: EventWriter<Save>, mut evw_exit: EventWriter<AppExit>) {
    evw_save.send(Save);
    info!("[EVENT] [WRITE] Save Game.");
    evw_exit.send(AppExit::Success);
}

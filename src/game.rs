use std::io::Result;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

use crate::{
    loading::BackgroundAssets,
    save::{format_load, format_save, Saveable},
    settings::Settings,
    AppState, Title,
};

pub struct GameLoopPlugin;
impl Plugin for GameLoopPlugin {
    fn name(&self) -> &str {
        "Game Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), startup)
            .add_systems(OnExit(AppState::Playing), cleanup)
            .insert_resource(
                PowerUnlockFlags::load("power_unlocks.ron")
                    .expect("[ERROR] Could not load power_unlocks.ron"),
            )
            .insert_resource(
                Powers::load("powers.ron").expect("[ERROR] Could not load powers.ron"),
            );
    }
}

#[derive(Component)]
struct CleanupGame;

#[derive(Component, Deserialize, Serialize)]
struct Power {
    title: Title,
    id: usize,
    cost: i64,
    production_amount: i64,
    production_rate: f64,
    max_owned: i64,
}

#[derive(Deref, DerefMut, Deserialize, Resource, Serialize)]
struct Powers(Vec<Power>);
impl Saveable for Powers {
    fn save(&self, filename: &str) -> std::io::Result<()> {
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}

#[derive(Deserialize, Resource, Serialize)]
pub struct PowerUnlockFlags(HashMap<usize, bool>);
impl Saveable for PowerUnlockFlags {
    fn save(&self, filename: &str) -> std::io::Result<()> {
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}

fn startup(
    mut commands: Commands,
    settings: Res<Settings>,
    background_assets: Res<BackgroundAssets>,
) {
    commands.spawn((
        Sprite::from_image(background_assets.room_background.clone()),
        Transform::from_scale(Vec3::splat(settings.sprite_scale())),
        CleanupGame,
    ));
}

fn cleanup(mut commands: Commands, query_entity: Query<Entity, With<CleanupGame>>) {
    for entity in query_entity.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Game Entities");
    }
}

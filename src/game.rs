use std::io::Result;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

use crate::{
    loading::{BackgroundAssets, PowerAssets},
    save::{format_load, format_save, Saveable},
    settings::Settings,
    ui::{UIButton, UIButtonParentNode, UIButtonPowerNode},
    AppState, Title, ID,
};

pub struct GameLoopPlugin;
impl Plugin for GameLoopPlugin {
    fn name(&self) -> &str {
        "Game Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPowerButton>()
            .add_systems(OnEnter(AppState::Playing), startup)
            .add_systems(OnExit(AppState::Playing), cleanup)
            .add_systems(
                Update,
                evr_spawn_power_button.run_if(in_state(AppState::Playing)),
            )
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
    id: ID,
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

#[derive(Event)]
struct SpawnPowerButton(usize);

fn startup(
    mut commands: Commands,
    settings: Res<Settings>,
    power_flags: Res<PowerUnlockFlags>,
    background_assets: Res<BackgroundAssets>,
    mut evr_spawn_power_button: EventWriter<SpawnPowerButton>,
) {
    // SPAWN BACKGROUND SPRITE
    commands.spawn((
        Sprite::from_image(background_assets.room_background.clone()),
        Transform::from_scale(Vec3::splat(settings.sprite_scale())),
        CleanupGame,
    ));

    // SPAWN POWER BUTTON NODE
    commands.spawn((
        UIButtonParentNode::default(),
        UIButtonParentNode::marker(),
        CleanupGame,
    ));

    for i in 0..power_flags.0.len() {
        if let Some((_k, v)) = power_flags.0.get_key_value(&i) {
            if *v {
                evr_spawn_power_button.send(SpawnPowerButton(i));
            }
        }
    }
}

fn cleanup(mut commands: Commands, query_entity: Query<Entity, With<CleanupGame>>) {
    for entity in query_entity.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Game Entities");
    }
}

fn evr_spawn_power_button(
    mut evr_spawn_power_button: EventReader<SpawnPowerButton>,
    mut query_parent_node: Query<Entity, With<UIButtonParentNode>>,
    power_assets: Res<PowerAssets>,
    query_power: Query<&Power>,
    mut commands: Commands,
    power_flags: Res<PowerUnlockFlags>,
) {
    for ev in evr_spawn_power_button.read() {
        // CHECK IF KEY/VALUE PAIR EXISTS
        if let Some((_k, v)) = power_flags.0.get_key_value(&ev.0) {
            // CHECK IF POWER IS UNLOCKED
            if *v {
                // CHECK IF POWER IS ALREADY SPAWNED
                if !query_power.iter().any(|power| power.id.0 == ev.0) {
                    // SAFELY GET PARENT NODE ENTITY
                    if let Ok(entity) = query_parent_node.get_single_mut() {
                        // SPAWN AND INSERT POWER BUTTON

                        let children_style = (
                            BorderColor(Color::NONE),
                            BorderRadius::ZERO,
                            BackgroundColor(Color::NONE),
                            ImageNode::from_atlas_image(
                                power_assets.power_atlas.clone(),
                                TextureAtlas {
                                    layout: power_assets.power_layout.clone(),
                                    index: ev.0,
                                },
                            ),
                        );

                        let grandchildren_style = (
                            BorderColor(Color::NONE),
                            BorderRadius::ZERO,
                            BackgroundColor(Color::NONE),
                            ImageNode::from_atlas_image(
                                power_assets.border_atlas.clone(),
                                TextureAtlas {
                                    layout: power_assets.border_layout.clone(),
                                    index: 0,
                                },
                            ),
                        );
                        let children = commands
                            .spawn((UIButtonPowerNode::default(), Button, children_style))
                            .id();
                        let grandchildren = commands
                            .spawn((
                                UIButtonPowerNode::default(),
                                Button,
                                UIButton,
                                grandchildren_style,
                            ))
                            .id();
                        commands.entity(children).add_children(&[grandchildren]);
                        commands.entity(entity).add_children(&[children]);

                        info!("[SPAWNED] Power Button: {}", ev.0);
                    } else {
                        info!("[ERROR] Button Parent Node Not Spawned.");
                    }
                } else {
                    info!("[ERROR] Power: {} Already Spawned", ev.0);
                }
            } else {
                info!("[ERROR] Power: {} Not Unlocked", ev.0);
            }
        }
    }
}

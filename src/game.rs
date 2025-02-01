use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::{
    loading::{BackgroundAssets, PowerAssets, UiAssets},
    save::{format_load, format_save, Saveable},
    settings::Settings,
    ui::{
        Pallette, PauseButton, PauseButtonChildNode, PauseButtonParentNode, PauseParentNode,
        PowerTextNode, ScreenButton, ScreenButtonNode, UIButton, UIButtonParentNode,
        UIButtonPowerNode,
    },
    AppState, Cost, CurrentOwned, MaxOwned, PauseState, ProdAmount, ProdRate, Title, ID,
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
            .add_systems(OnEnter(PauseState::Paused), pause_startup)
            .add_systems(OnExit(PauseState::Paused), pause_cleanup)
            .add_systems(Update, pause_click.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                (evr_spawn_power_button, screen_click, update_power_text)
                    .run_if(in_state(PauseState::Unpaused)),
            )
            .insert_resource(
                PowerUnlockFlags::load("power_unlocks.ron")
                    .expect("[ERROR] Could not load power_unlocks.ron"),
            )
            .insert_resource(Powers::load("powers.ron").expect("[ERROR] Could not load powers.ron"))
            .insert_resource(
                TotalPower::load("total_power.ron")
                    .expect("[ERROR] Could not load total_power.ron"),
            );
    }
}

#[derive(Component)]
struct CleanupGame;

#[derive(Component)]
struct CleanupPause;

#[derive(Default, Deref, DerefMut, Deserialize, Resource, Serialize)]
pub struct TotalPower(i64);
impl TotalPower {
    fn add_power(&mut self, i: i64) {
        self.0 += i;
    }

    fn power(&self) -> i64 {
        self.0
    }
}
impl Saveable for TotalPower {
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

#[derive(Component, Deserialize, Serialize)]
struct PowerText;

#[derive(Component, Deserialize, Serialize)]
struct Power {
    title: Title,
    id: ID,
    cost: Cost,
    production_amount: ProdAmount,
    production_rate: ProdRate,
    max_owned: MaxOwned,
    current_owned: CurrentOwned,
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
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    power_flags: Res<PowerUnlockFlags>,
    background_assets: Res<BackgroundAssets>,
    ui_assets: Res<UiAssets>,
    mut evr_spawn_power_button: EventWriter<SpawnPowerButton>,
) {
    // SPAWN BACKGROUND SPRITE
    commands.spawn((
        Sprite::from_image(background_assets.room_background.clone()),
        Transform::from_scale(Vec3::splat(settings.sprite_scale())),
        CleanupGame,
    ));

    // SPAWN SCREEN CLICK NODE
    let screen_button_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
    );

    commands.spawn((
        ScreenButtonNode::default(),
        ScreenButtonNode::marker(),
        Button,
        ScreenButton,
        screen_button_style,
        CleanupGame,
    ));

    // SPAWN POWER BUTTON PARENT NODE
    commands.spawn((
        UIButtonParentNode::default(),
        UIButtonParentNode::marker(),
        CleanupGame,
    ));

    // SPAWN TOTAL POWER TEXT
    let font: Handle<Font> = asset_server.load("fonts/PublicPixel.ttf");
    let text_color = Pallette::White.srgb();
    let font_size = 60.0;
    commands
        .spawn((
            PowerTextNode::default(),
            PowerTextNode::marker(),
            Text::new("POWER: "),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(text_color.clone()),
            BackgroundColor(Color::NONE),
            CleanupGame,
        ))
        .with_child((
            TextSpan::default(),
            (
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                TextColor(text_color),
            ),
            PowerText,
        ));

    // SPAWN PAUSE BUTTON
    let entity = commands
        .spawn((
            PauseButtonParentNode::default(),
            PauseButtonParentNode::marker(),
            CleanupGame,
        ))
        .id();

    let children_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
        ImageNode::from_atlas_image(
            ui_assets.pause_atlas.clone(),
            TextureAtlas {
                layout: ui_assets.pause_layout.clone(),
                index: 0,
            },
        ),
    );

    let grandchildren_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
        ImageNode::from_atlas_image(
            ui_assets.pause_border_atlas.clone(),
            TextureAtlas {
                layout: ui_assets.pause_border_layout.clone(),
                index: 0,
            },
        ),
    );
    let children = commands
        .spawn((PauseButtonChildNode::default(), children_style))
        .id();
    let grandchildren = commands
        .spawn((
            PauseButtonChildNode::default(),
            PauseButtonChildNode::marker(),
            Button,
            UIButton,
            PauseButton,
            grandchildren_style,
        ))
        .id();
    commands.entity(children).add_children(&[grandchildren]);
    commands.entity(entity).add_children(&[children]);

    info!("[SPAWNED] Game Nodes");

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
    powers: Res<Powers>,
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

                        info!("[SPAWNED] Power + Button: {}", ev.0);
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

fn screen_click(
    mut total_power: ResMut<TotalPower>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ScreenButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            total_power.add_power(1);
            info!("[EVENT] Click");
            info!("[MODIFIED] Total Power: {}", total_power.0);
        }
    }
}

fn pause_click(
    pause_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match pause_state.get() {
                PauseState::Paused => {
                    next_state.set(PauseState::Unpaused);
                    info!("[MODIFIED] PauseState >> Unpaused");
                }
                PauseState::Unpaused => {
                    next_state.set(PauseState::Paused);
                    info!("[MODIFIED] PauseState >> Paused");
                }
            }
        }
    }
}

fn update_power_text(
    total_power: Res<TotalPower>,
    mut query_power_text: Query<&mut TextSpan, With<PowerText>>,
) {
    if total_power.is_changed() {
        for mut span in &mut query_power_text {
            **span = format!("{}", total_power.power());
        }
    }
}

fn pause_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PublicPixel.ttf");
    let parent_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    );

    commands
        .spawn((
            PauseParentNode::default(),
            PauseParentNode::marker(),
            parent_style,
            CleanupPause,
        ))
        .with_child((
            Text::from("PAUSED"),
            TextFont {
                font,
                font_size: 60.0,
                ..default()
            },
            TextColor(Pallette::Lighter.srgb()),
        ));
    info!("[SPAWNED] Pause Entities");
}

fn pause_cleanup(mut commands: Commands, query_entity: Query<Entity, With<CleanupPause>>) {
    for entity in query_entity.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Pause Entities");
    }
}

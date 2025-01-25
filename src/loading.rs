use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn name(&self) -> &str {
        "Loading Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Menu)
                .load_collection::<BackgroundAssets>()
                .load_collection::<MusicAssets>(),
        )
        .add_systems(OnEnter(AppState::Loading), startup)
        .add_systems(OnExit(AppState::Loading), cleanup);
    }
}

#[derive(Component)]
struct CleanupTitleScreen;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/windless_slopes.ogg")),
        PlaybackSettings::LOOP,
    ));
    info!("[SPAWNED] Title Audio.");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupTitleScreen>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(dead_code)] // TODO:
#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(path = "sprites/backgrounds/title.png")]
    title_background: Handle<Image>,

    #[asset(path = "sprites/backgrounds/room.png")]
    room_background: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MusicAssets {
    #[asset(path = "audio/windless_slopes.ogg")] //TODO: This is placeholder non-DMCA audio
    title_music: Handle<AudioSource>,
}

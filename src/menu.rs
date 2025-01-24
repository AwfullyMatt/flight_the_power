use bevy::prelude::*;

use crate::settings::{Pallette, Settings};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn name(&self) -> &str {
        "Menu Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    //TODO: Title Card Before Menu

    // SPAWN BACKGROUND
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/backgrounds/room.png")),
        Transform::from_scale(Vec3::splat(settings.resolution.scale())),
    ));

    // SPAWN BUTTONS
    // TODO: Programmatic Button Size
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(25.0),
            top: Val::Percent(70.0),
            bottom: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::SpaceEvenly,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Percent(10.)),
                    BackgroundColor(Pallette::DARK.srgb()),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::LIGHT.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::SpaceEvenly,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Percent(10.)),
                    BackgroundColor(Pallette::DARK.srgb()),
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::LIGHT.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::SpaceEvenly,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Percent(10.)),
                    BackgroundColor(Pallette::DARK.srgb()),
                ))
                .with_child((
                    Text::new("Exit"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::LIGHT.srgb()),
                ));
        });
}

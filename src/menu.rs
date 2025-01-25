use bevy::prelude::*;

use crate::{
    settings::{Pallette, Settings},
    AppState,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn name(&self) -> &str {
        "Menu Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(
                Update,
                menu_button_interaction.run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), cleanup);
    }
}

#[derive(Component)]
struct CleanupMainMenu;

#[derive(Component)]
pub enum MainMenuButton {
    Play,
    Settings,
    Exit,
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
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                top: Val::Percent(70.0),
                bottom: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            CleanupMainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    MainMenuButton::Play,
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
                    MainMenuButton::Settings,
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
                    MainMenuButton::Exit,
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

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupMainMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &MainMenuButton,
        ),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut text_color_query: Query<&mut TextColor>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color, mut border_color, children, mmb) in
        &mut interaction_query
    {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                *text_color = Pallette::LIGHTER.srgb().into();
                *background_color = Pallette::DARK.srgb().into();
                border_color.0 = Pallette::DARKER.srgb().into();
            }
            Interaction::Pressed => {
                *text_color = Pallette::LIGHT.srgb().into();
                *background_color = Pallette::DARKER.srgb().into();
                border_color.0 = Pallette::BLACK.srgb().into();

                match mmb {
                    MainMenuButton::Play => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Playing);
                        }
                    }
                    MainMenuButton::Settings => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Settings);
                        }
                    }
                    MainMenuButton::Exit => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Exit);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *text_color = Pallette::WHITE.srgb().into();
                *background_color = Pallette::LIGHT.srgb().into();
                border_color.0 = Pallette::DARK.srgb().into();
            }
        }
    }
}

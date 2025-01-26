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
        app.add_systems(OnEnter(AppState::Menu), startup)
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
        Sprite::from_image(asset_server.load("sprites/backgrounds/title.png")),
        Transform::from_scale(Vec3::splat(settings.resolution.scale())),
        CleanupMainMenu,
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
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb().into()),
                    MainMenuButton::Play,
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb().into()),
                    MainMenuButton::Settings,
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(115.0),
                        border: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BorderColor(Pallette::Black.srgb()),
                    BorderRadius::all(Val::Percent(10.0)),
                    BackgroundColor(Pallette::Lighter.srgb().into()),
                    MainMenuButton::Exit,
                ))
                .with_child((
                    Text::new("Exit"),
                    TextFont {
                        font: asset_server.load("fonts/PublicPixel.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Pallette::Black.srgb()),
                ));
        });
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupMainMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[CLEANED] Main Menu.");
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
                background_color.0 = Pallette::Lighter.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
            Interaction::Hovered => {
                background_color.0 = Pallette::Darker.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
            Interaction::Pressed => {
                background_color.0 = Pallette::Darker.srgb();
                border_color.0 = Pallette::Black.srgb();
                text_color.0 = Pallette::Black.srgb();

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
        }
    }
}

use bevy::prelude::*;

use crate::{
    loading::BackgroundAssets,
    settings::Settings,
    ui::{Pallette, UIButton, UIButtonParentNode},
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

fn startup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    settings: Res<Settings>,
    background_assets: Res<BackgroundAssets>,
) {
    //TODO: Title Card Before Menu

    // SPAWN BACKGROUND
    commands.spawn((
        Sprite::from_image(background_assets.title_background.clone()),
        Transform::from_scale(Vec3::splat(settings.sprite_scale())),
        CleanupMainMenu,
    ));

    // SPAWN BUTTONS
    // TODO: Programmatic Button Size
    let font = asset_server.load("fonts/PublicPixel.ttf");

    let child_node = Node {
        width: Val::Px(320.0),
        height: Val::Px(115.0),
        border: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let style = (
        BorderColor(Pallette::Black.srgb()),
        BorderRadius::all(Val::Percent(10.0)),
        BackgroundColor(Pallette::Lighter.srgb().into()),
    );

    commands
        .spawn((UIButtonParentNode::node(), CleanupMainMenu))
        .with_children(|parent| {
            for i in 0..3 {
                let text: Text = match i {
                    0 => Text::new("Play"),
                    1 => Text::new("Settings"),
                    _ => Text::new("Exit"),
                };
                let mmb: MainMenuButton = match i {
                    0 => MainMenuButton::Play,
                    1 => MainMenuButton::Settings,
                    _ => MainMenuButton::Exit,
                };

                parent
                    .spawn((child_node.clone(), Button, mmb, UIButton, style.clone()))
                    .with_child((
                        text,
                        TextFont {
                            font: font.clone(),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Pallette::Black.srgb()),
                    ));
            }
        });
    info!("[SPAWNED] Settings Menu Entities.");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupMainMenu>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[CLEANED] Main Menu.");
}

fn menu_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mmb) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => match mmb {
                MainMenuButton::Play => {
                    next_state.set(AppState::Playing);
                    info!("[MODIFIED] AppState >> Playing");
                }
                MainMenuButton::Settings => {
                    next_state.set(AppState::Settings);
                    info!("[MODIFIED] AppState >> Settings");
                }
                MainMenuButton::Exit => {
                    next_state.set(AppState::Exit);
                    info!("[MODIFIED] AppState >> Exit");
                }
            },
            _ => {}
        }
    }
}

use bevy::prelude::*;

use crate::PauseState;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn name(&self) -> &str {
        "UI Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (button_color_system_unpaused, button_atlas_system_unpaused)
                .run_if(not(in_state(PauseState::Paused))),
        )
        .add_systems(
            Update,
            (button_color_system_paused, button_atlas_system_paused)
                .run_if(in_state(PauseState::Paused)),
        );
    }
}

#[derive(Component)]
pub struct UIButton;

#[derive(Component)]
pub struct ScreenButton;

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct PowerButton;

#[derive(Component)]
pub struct SaveExitButton;

#[derive(Component)]
pub struct PauseParentNode;
impl PauseParentNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(70.0),
            top: Val::Percent(15.0),
            left: Val::Percent(0.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct PauseChildNode;
impl PauseChildNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(20.0),
            top: Val::Percent(80.0),
            left: Val::Percent(60.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct PauseButtonParentNode;
impl PauseButtonParentNode {
    pub fn default() -> Node {
        Node {
            width: Val::Px(60.0),
            height: Val::Px(60.0),
            top: Val::Percent(3.0),
            left: Val::Percent(95.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct PauseButtonChildNode;
impl PauseButtonChildNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct PowerTextNode;
impl PowerTextNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            top: Val::Percent(5.0),
            left: Val::Percent(10.0),
            right: Val::Percent(10.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct ScreenButtonNode;
impl ScreenButtonNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(65.0),
            top: Val::Percent(10.0),
            left: Val::Percent(10.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            ..default()
        }
    }
    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct UIButtonParentNode;
impl UIButtonParentNode {
    pub fn node() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(30.0),
            top: Val::Percent(75.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        }
    }

    pub fn new(w: f32, h: f32, t: f32) -> Node {
        Node {
            width: Val::Percent(w),
            height: Val::Percent(h),
            top: Val::Percent(t),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        }
    }

    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct UIButtonChildNode;
#[allow(dead_code)] //TODO:
impl UIButtonChildNode {
    pub fn node() -> Node {
        Node {
            width: Val::Px(320.0),
            height: Val::Px(115.0),
            border: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }

    pub fn new(w: f32, h: f32, t: f32) -> Node {
        Node {
            width: Val::Percent(w),
            height: Val::Percent(h),
            top: Val::Percent(t),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }

    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct UIButtonPowerNode;
#[allow(dead_code)] //TODO:
impl UIButtonPowerNode {
    pub fn node() -> Node {
        Node {
            width: Val::Px(160.0),
            height: Val::Px(160.0),
            border: UiRect::all(Val::Px(0.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    pub fn marker() -> Self {
        Self
    }
}

#[derive(Component)]
pub struct UiButtonInfoNode;
#[allow(dead_code)] //TODO:
impl UiButtonInfoNode {
    pub fn node() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            bottom: Val::Percent(75.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }

    pub fn marker() -> Self {
        Self
    }
}

#[allow(dead_code)] //TODO:
#[derive(Resource)]
pub enum Pallette {
    White,
    Lighter,
    Light,
    Dark,
    Darker,
    Black,
}
impl Pallette {
    pub fn srgb(&self) -> Color {
        use Pallette::*;
        match self {
            White => Color::srgb(1., 1., 1.),
            Lighter => Color::srgb(0.8275, 0.8275, 0.8275),
            Light => Color::srgb(0.06549, 0.06549, 0.06549),
            Dark => Color::srgb(0.3647, 0.3647, 0.3647),
            Darker => Color::srgb(0.2118, 0.2118, 0.2118),
            Black => Color::srgb(0., 0., 0.),
        }
    }
}

fn button_color_system_unpaused(
    mut query_interaction: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<UIButton>,
            Without<UIButtonPowerNode>,
        ),
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, mut border_color, children) in &mut query_interaction {
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
                border_color.0 = Pallette::White.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
        }
    }
}

fn button_color_system_paused(
    mut query_interaction: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<SaveExitButton>,
            Without<UIButtonPowerNode>,
        ),
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, mut border_color, children) in &mut query_interaction {
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
                border_color.0 = Pallette::White.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
        }
    }
}

fn button_atlas_system_unpaused(
    mut query_interaction: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<UIButton>),
    >,
) {
    for (interaction, mut image_node) in query_interaction.iter_mut() {
        match *interaction {
            Interaction::None => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 0;
                }
            }
            Interaction::Hovered => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 1;
                }
            }
            Interaction::Pressed => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 2;
                }
            }
        }
    }
}

fn button_atlas_system_paused(
    mut query_interaction: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<UIButton>, With<PauseButton>),
    >,
) {
    for (interaction, mut image_node) in query_interaction.iter_mut() {
        match *interaction {
            Interaction::None => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 0;
                }
            }
            Interaction::Hovered => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 1;
                }
            }
            Interaction::Pressed => {
                if let Some(atlas) = &mut image_node.texture_atlas {
                    atlas.index = 2;
                }
            }
        }
    }
}

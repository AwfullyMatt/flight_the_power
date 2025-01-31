use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn name(&self) -> &str {
        "UI Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_color_system, button_atlas_system));
    }
}

#[derive(Component)]
pub struct UIButton;

#[derive(Component)]
pub struct UIButtonParentNode;
impl UIButtonParentNode {
    pub fn default() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(33.0),
            top: Val::Percent(70.0),
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
    pub fn default() -> Node {
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
    pub fn default() -> Node {
        Node {
            width: Val::Px(160.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    pub fn new(w: f32, h: f32, b: f32) -> Node {
        Node {
            width: Val::Percent(w),
            height: Val::Percent(h),
            border: UiRect::all(Val::Px(b)),
            align_items: AlignItems::Center,
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

fn button_color_system(
    mut query_interaction: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<UIButton>),
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

fn button_atlas_system(
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

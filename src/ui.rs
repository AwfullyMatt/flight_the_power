use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn name(&self) -> &str {
        "UI Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_color_system);
    }
}

#[derive(Component)]
pub struct UIButton;

#[allow(dead_code)]
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
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &UIButton,
        ),
        (Changed<Interaction>, With<UIButton>),
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, mut border_color, children, _smb) in
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
                border_color.0 = Pallette::White.srgb();
                text_color.0 = Pallette::Black.srgb();
            }
        }
    }
}

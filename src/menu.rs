use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn name(&self) -> &str {
        "Menu Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {}

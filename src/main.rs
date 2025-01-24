use bevy::app::{App, AppExit};
use flightthepower::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}

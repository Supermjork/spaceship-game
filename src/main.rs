mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // User Defined Plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}

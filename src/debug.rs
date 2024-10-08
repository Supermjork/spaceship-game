use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    // Logging ID + Position for each entity
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at Position ({:?}).",
            entity, transform.translation
        );
    }
}

use bevy::prelude::*;

/// Main player sprite.
const PLAYER_SPRITE: &str = "player_a_01.png";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rustacean Invasion".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}

use bevy::prelude::*;

fn draw_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(19.0, 19.0)),
            image: asset_server.load("default_tile.png"),
            ..default()
        },
        Transform::default(),
    ));
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_tile);
    }
}
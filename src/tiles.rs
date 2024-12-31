use bevy::prelude::*;

fn draw_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Parent white square (19x19)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(19.0, 19.0))),
        MeshMaterial2d(materials.add(Color::hsla(0.0, 0.0, 1.0, 1.0))),
    ))
        .with_children(|parent| {
            // Child black square (1x1)
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                MeshMaterial2d(materials.add(Color::hsla(0.0, 0.0, 0.0, 1.0))),
            ));
        });
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_tile);
    }
}
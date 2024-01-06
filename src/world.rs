use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.8, 0.8, 0.8),
        brightness: 0.2,
    });
}

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speed(5.0),
        Name::new("Player"),
    );
    commands.spawn(player);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let cam = cam_q.get_single().expect("There should be exactly one cam");
    for mut transform in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }
        if keys.pressed(KeyCode::Space) {
            direction += cam.up();
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            direction += cam.down();
        }
        let movement = direction.normalize_or_zero() * time.delta_seconds() * 5.0;
        transform.translation += movement;
    }
}

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::CursorGrabMode,
};

const CAMERA_SPEED: f32 = 0.1;
const ZOOM_SPEED: f32 = 1.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (grab_mouse, move_camera, rotate_camera, zoom_camera));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn move_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut cam_q: Query<&mut Transform, With<Camera3d>>,
) {
    let mut cam = cam_q
        .get_single_mut()
        .expect("There should be exactly one camera");
    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::Up) {
        direction += cam.forward();
    }
    if keys.pressed(KeyCode::Down) {
        direction += cam.back();
    }
    if keys.pressed(KeyCode::Left) {
        direction += cam.left();
    }
    if keys.pressed(KeyCode::Right) {
        direction += cam.right();
    }
    cam.translation += direction * time.delta_seconds() * CAMERA_SPEED * 10.0;
}

fn rotate_camera(
    time: Res<Time>,
    mut mouse: EventReader<MouseMotion>,
    mut cam_q: Query<&mut Transform, With<Camera3d>>,
) {
    let mut cam = cam_q
        .get_single_mut()
        .expect("There should be exactly one camera");
    let looking_at = Vec3::ZERO;
    for ev in mouse.read() {
        let delta = ev.delta * CAMERA_SPEED * time.delta_seconds();
        let rotation = Quat::from_rotation_y(-delta.x) * Quat::from_rotation_x(-delta.y);
        cam.rotate_around(looking_at, rotation);
    }
}

fn zoom_camera(
    mut mouse: EventReader<MouseWheel>,
    mut cam_q: Query<&mut Transform, With<Camera3d>>,
) {
    let mut cam = cam_q
        .get_single_mut()
        .expect("There should be exactly one camera");
    let forward = cam.forward();
    for ev in mouse.read() {
        cam.translation += forward * ev.y * ZOOM_SPEED;
    }
}

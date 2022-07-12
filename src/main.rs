use bevy::{prelude::*, input::mouse::MouseMotion, math::vec3};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;


#[derive(Component)]
struct Cube;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec3);

#[derive(Component)]
struct Camera;

// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct MouseMovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MouseMovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}
/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}
/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}
fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}


fn main() {
    App::new()
        .init_resource::<InputState>()
        .init_resource::<MouseMovementSettings>()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(initial_grab_cursor)
        .add_system_set(
            SystemSet::new()
                .with_system(move_cube)
                .with_system(move_camera) // Comment for third person camera unattatched from cube
                .with_system(mouse_look) // Comment for third person camera unattatched from cube
                .with_system(apply_velocity.after(move_camera))
        )
        .add_system(cursor_grab)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands
        .spawn()
        .insert(Cube)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.3, rings: 2, depth: 0.7, latitudes: 200, longitudes: 300, uv_profile: shape::CapsuleUvProfile::Fixed })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Velocity(Vec3::ZERO));


    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn()
        .insert(Camera)
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Velocity(Vec3::ZERO));
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {

        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
        transform.translation.z += velocity.z * TIME_STEP;

        if transform.translation.y > 0.5 {
            velocity.y -= 0.1;
        } else if transform.translation.y < 0.5 {
            velocity.y = 0.;
            transform.translation.y = 0.5;
        }
        
        if velocity.x > 0.5 {
            velocity.x -= 0.5;
        } else if velocity.x < -0.5 {
            velocity.x += 0.5;
        } else { velocity.x = 0.}

        if velocity.z > 0.5 {
            velocity.z -= 0.5;
        } else if velocity.z < -0.5 {
            velocity.z += 0.5;
        } else { velocity.z = 0. }

        // todo!("Normalize movement vector")
        // dbg!(&transform.translation);
        //velocity.0 = velocity.normalize_or_zero();
        //dbg!(velocity.0);

    }
}

fn mouse_look(
    settings: Res<MouseMovementSettings>,
    mut state: ResMut<InputState>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Cube>)>,
    mut motion_evr: EventReader<MouseMotion>,
    mut cube_query: Query<&mut Transform, (With<Cube>, Without<Camera>)>,
    windows: Res<Windows>,

) {
    let window = windows.get_primary().unwrap();

    // Handle mouse events:
    for ev in motion_evr.iter() {
        let mut cam_transform = camera_query.single_mut();
        let mut cube_transform = cube_query.single_mut();

        // Only change pitch yaw if focused
        if window.cursor_locked() {
            let window_scale = window.height().min(window.width());
            state.pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
            state.yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
        }
        
        // Stop player from looking upside down
        state.pitch = state.pitch.clamp(-1.54, 1.54);

        cam_transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
            * Quat::from_axis_angle(Vec3::X, state.pitch);
        
        cube_transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
    }

}

#[allow(clippy::type_complexity)]
fn move_camera(
    state: ResMut<InputState>,
    mut camera_query: Query<(&mut Transform, &mut Velocity), (With<Camera>, Without<Cube>)>,
    cube_query: Query<(&Transform, &Velocity), (With<Cube>, Without<Camera>)>,
) {
    for (mut transform, mut velocity) in camera_query.iter_mut() {
        let (cube_t, cube_v) = cube_query.single();
        
        let i = cube_t.local_x(); let a = Vec3::ONE;
        let j = cube_t.local_y(); let b = vec3(1.0,1.0,1.0);
        let k = cube_t.local_z(); let c = vec3(3.,3.,3.);

        transform.translation = cube_t.translation + (a*i + b*j + c*k);
        velocity.0 = cube_v.0;
    }
}

fn move_cube(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Cube>>,
) {
    for (transform, mut velocity) in query.iter_mut() {

        // fixme: use velocity instead of translation
        // todo: change controls to tank controls (more like an fps)

        if keyboard_input.pressed(KeyCode::A) {
            let a = transform.left().to_array();
            velocity.x += a[0];
            velocity.y += a[1];
            velocity.z += a[2];
        }

        if keyboard_input.pressed(KeyCode::D) {
            let a = transform.right().to_array();
            velocity.x += a[0];
            velocity.y += a[1];
            velocity.z += a[2];
        }

        if keyboard_input.pressed(KeyCode::S) {
            let a = transform.back().to_array();
            velocity.x += a[0];
            velocity.y += a[1];
            velocity.z += a[2];
        }

        if keyboard_input.pressed(KeyCode::W) {
            let a = transform.forward().to_array();
            velocity.x += a[0];
            velocity.y += a[1];
            velocity.z += a[2];
        }
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.y += 0.5;
        }
    }

}
use bevy::prelude::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;


#[derive(Component)]
struct Cube;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec3);

#[derive(Component)]
struct Camera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_system(move_cube)
                .with_system(move_camera)
                .with_system(apply_velocity.after(move_camera))
        )
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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands
        .spawn()
        .insert(Cube)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
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
        
        if velocity.x > 0. {
            velocity.x -= 0.5;
        } else if velocity.x < 0. {
            velocity.x += 0.5;
        }

        if velocity.z > 0. {
            velocity.z -= 0.5;
        } else if velocity.z < 0. {
            velocity.z += 0.5;
        }

        // todo!("Normalize movement vector")
        // dbg!(&transform.translation);
        //velocity.0 = velocity.normalize_or_zero();
        //dbg!(velocity.0);

    }
}

// Todo: move camera to player
#[allow(dead_code)]
fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut Velocity), (With<Camera>, Without<Cube>)>,
    cube_query: Query<(&Transform, &Velocity), (With<Cube>, Without<Camera>)>,
) {
    for (mut transform, mut velocity) in camera_query.iter_mut() {
        let (cube_t, cube_v) = cube_query.single();
        
        transform.translation = cube_t.translation + Vec3::new(0., 1., 0.);
        velocity.0 = cube_v.0;
    }
}

fn move_cube(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Cube>>,
) {
    for (mut _transform, mut velocity) in query.iter_mut() {
        
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            velocity.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            velocity.z += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            velocity.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.y += 0.5;
        }
    }

}
mod boid;

use bevy::{
    prelude::*,
};
use bevy::prelude::TimerMode::Repeating;
use rand::random;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(boid::BoidTimer(Timer::from_seconds(0.1, Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, boid::update_boids)
        .run();
}

const X_EXTENT: f32 = 40.0;
/// Randomly Instantiate some amount of Boids
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 64.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    let num_birds = 100;

    let sphere_handle = meshes.add(Sphere::default());
    let material_handle = materials.add(Color::rgb(0.7, 0.7, 0.7));

    // Generate our flock
    for _ in 0..num_birds {
        // Get random position for bird
        let x = random::<f32>() * X_EXTENT - (X_EXTENT / 2.0);
        let y = random::<f32>() * X_EXTENT - (X_EXTENT / 2.0);
        let z = random::<f32>() * X_EXTENT - (X_EXTENT / 2.0);
        let p_vec = Vec3::new(x, y, z);

        commands.spawn((
            boid::Boid,
            boid::Position { p: p_vec },
            boid::Velocity {
                v: Vec3::new(
                    random::<f32>() * 2.0 - 1.0,
                    random::<f32>() * 2.0 - 1.0,
                    random::<f32>() * 2.0 - 1.0
                )
            },
            PbrBundle {
                mesh: sphere_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_translation(p_vec),
                ..Default::default()
            }
        ));
    }
}
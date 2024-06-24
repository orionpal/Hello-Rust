use bevy::prelude::*;

/// Label for Boid type entities
///
/// Should be paired with:
/// - Velocity
/// - Position
#[derive(Component)]
pub struct Boid;

#[derive(Component)]
pub struct Velocity {pub v: Vec3}

#[derive(Component)]
pub struct Position {pub p: Vec3}

#[derive(Resource)]
pub struct BoidTimer(pub Timer);

const PROXIMITY: f32 = 4.0;
pub fn update_boids(mut boids: Query<(&mut Transform, &mut Velocity), With<Boid>>) {
    let boids_vec: Vec<(Vec3, Vec3)> = boids
        .iter()
        .map(|(transform, velocity)| (transform.translation, velocity.v))
        .collect();
    // FOR b in Boids
    for (mut transform, mut velocity) in &mut boids {
        // println!("bird position: {} bird velocity: {}", transform.translation, velocity.v);
        let position = transform.translation;
        let cohesion_vector = cohesion(position, &boids_vec);
        let seperation_vector = seperation(position, &boids_vec);
        let alignment_vector = alignment(position, velocity.v, &boids_vec);

        velocity.v += cohesion_vector +
            seperation_vector +
            alignment_vector;
            // gravity();
        velocity.v = speed_limit(velocity.v);
        in_boundary(position, velocity.v);
        transform.translation += velocity.v;
        // println!("bird new position: {} bird new velocity: {}", transform.translation, velocity.v);
    }
}

/// Try going towards the center of mass
/// each element of boids should have (transform.translation, velocity)
fn cohesion(b_position: Vec3, boids: &Vec<(Vec3, Vec3)>) -> Vec3 {
    let mut num_boids = 0.0 as f32;
    const CENTERING_FACTOR: f32 = 0.05; // adjust velocity by this %
    let mut result = Vec3::new(0.0, 0.0, 0.0);
    for (other_position, _) in boids {
        if b_position != *other_position {
            result += *other_position;
            num_boids += 1.0;
        }
    }
    if num_boids > 0.0 {
        result = result / num_boids; // Standardize
    }
    return (result - b_position) * CENTERING_FACTOR; // percentage towards b_position
}

fn seperation(b_position: Vec3, boids: &Vec<(Vec3, Vec3)>) -> Vec3 {
    let mut result = Vec3::new(0.0, 0.0, 0.0);
    const MIN_DIST: f32 = 5.0; // The distance to stay away from other boids
    const AVOID_FACTOR: f32 = 0.05; // Adjust velocity by this %
    for (other_position, _) in boids {
        if b_position != *other_position && (b_position-*other_position).length() < MIN_DIST {
            result += (b_position - *other_position);
        }
    }
    return result * AVOID_FACTOR;
}

fn alignment(b_position: Vec3, b_velocity: Vec3, boids: &Vec<(Vec3, Vec3)>) -> Vec3 {
    let mut num_boids = 0.0;
    const ALIGNMENT_FACTOR: f32= 0.05;
    let mut avg_velocity = Vec3::new(0.0, 0.0, 0.0);

    for (other_position, other_velocity) in boids {
        if b_position != *other_position && (b_position-*other_position).length() < PROXIMITY  {
            avg_velocity += *other_velocity;
            num_boids += 1.0;
        }
    }

    if num_boids > 0.0 {
        avg_velocity = avg_velocity / num_boids; // Standardize
    }
    return (avg_velocity - b_velocity) * ALIGNMENT_FACTOR; // percentage towards b_position
}


fn speed_limit(b_velocity: Vec3) -> Vec3 {
    let max_v = 2.0;
    if b_velocity.length() > max_v {
        return (b_velocity / b_velocity.length()) * max_v
    }
    return b_velocity
}

fn in_boundary(b_position: Vec3, mut b_velocity: Vec3) {
    const MARGIN: f32 = 20.0;
    const TURNFACTOR: f32 = 1.0;

    if (b_position.x < MARGIN) {
        b_velocity.x += TURNFACTOR;
    }
    if (b_position.x > -MARGIN) {
        b_velocity.x -= TURNFACTOR;
    }
    if (b_position.y < MARGIN) {
        b_velocity.y += TURNFACTOR;
    }
    if (b_position.y > -MARGIN) {
        b_velocity.y -= TURNFACTOR;
    }
    if (b_position.z < MARGIN) {
        b_velocity.z += TURNFACTOR;
    }
    if (b_position.z > -MARGIN) {
        b_velocity.z -= TURNFACTOR;
    }
}
fn gravity() -> Vec3 {
    return Vec3::new(0.0, -0.1, 0.0);
}
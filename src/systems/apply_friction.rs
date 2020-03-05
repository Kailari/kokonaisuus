use crate::components::{FrictionComponent, VelocityComponent};

// The same as before, but the logic now uses `Vector` methods
pub fn apply_friction(velocities: &mut Vec<VelocityComponent>, frictions: &Vec<FrictionComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut fri_iter = frictions.iter();

    while let (Some(vel), Some(fri)) = (vel_iter.next(), fri_iter.next()) {
        if vel.value.length_squared() < f64::EPSILON {
            continue;
        }

        let friction = vel.value.normalize().abs() * fri.value;
        let magnitude = (vel.value.abs() - friction).max(0.0);
        vel.value.x = vel.value.x.signum() * magnitude.x;
        vel.value.y = vel.value.y.signum() * magnitude.y;
    }
}
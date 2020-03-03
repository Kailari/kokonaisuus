#![feature(assoc_int_consts)]

struct PositionComponent {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for PositionComponent {
    fn from(source: (f64, f64)) -> Self {
        PositionComponent { x: source.0, y: source.1 }
    }
}

struct VelocityComponent {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for VelocityComponent {
    fn from(source: (f64, f64)) -> Self {
        VelocityComponent { x: source.0, y: source.1 }
    }
}

struct FrictionComponent {
    amount: f64,
}

impl From<f64> for FrictionComponent {
    fn from(source: f64) -> Self {
        FrictionComponent { amount: source }
    }
}

struct AccelerationComponent {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for AccelerationComponent {
    fn from(source: (f64, f64)) -> Self {
        AccelerationComponent { x: source.0, y: source.1 }
    }
}

pub fn main() {
    let mut positions = vec![
        PositionComponent::from((0.0, 0.0)),
        PositionComponent::from((-42.0, -42.0)),
        PositionComponent::from((234.0, 123.0)),
        PositionComponent::from((6.0, 9.0)),
    ];
    let mut velocities = vec![
        VelocityComponent::from((40.0, 10.0)),
        VelocityComponent::from((30.0, 20.0)),
        VelocityComponent::from((20.0, 30.0)),
        VelocityComponent::from((10.0, 40.0)),
    ];
    let frictions = vec![
        FrictionComponent::from(1.0),
        FrictionComponent::from(2.0),
        FrictionComponent::from(3.0),
        FrictionComponent::from(4.0),
    ];
    let accelerations = vec![
        AccelerationComponent::from((2.0, 16.0)),
        AccelerationComponent::from((4.0, 2.0)),
        AccelerationComponent::from((8.0, 4.0)),
        AccelerationComponent::from((16.0, 8.0)),
    ];

    apply_acceleration(&mut velocities, &accelerations);
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    print_positions(&positions)
}

fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut acc_iter = accelerations.iter();

    while let (Some(vel), Some(acc)) = (vel_iter.next(), acc_iter.next()) {
        vel.x += acc.x;
        vel.y += acc.y;
    }
}

fn apply_friction(velocities: &mut Vec<VelocityComponent>, frictions: &Vec<FrictionComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut fri_iter = frictions.iter();

    while let (Some(vel), Some(fri)) = (vel_iter.next(), fri_iter.next()) {
        let velocity_length_squared = vel.x * vel.x + vel.y * vel.y;

        if velocity_length_squared < f64::EPSILON {
            continue;
        }

        let velocity_length = velocity_length_squared.sqrt();
        let abs_friction_x = (vel.x / velocity_length * fri.amount).abs();
        let abs_friction_y = (vel.y / velocity_length * fri.amount).abs();

        let magnitude_x = (vel.x.abs() - abs_friction_x).max(0.0);
        let magnitude_y = (vel.x.abs() - abs_friction_y).max(0.0);
        vel.x = vel.x.signum() * magnitude_x;
        vel.y = vel.y.signum() * magnitude_y;
    }
}

fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let mut pos_iter = positions.iter_mut();
    let mut vel_iter = velocities.iter();

    while let (Some(pos), Some(vel)) = (pos_iter.next(), vel_iter.next()) {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn print_positions(positions: &Vec<PositionComponent>) {
    let mut value_iter = positions.iter();

    while let Some(value) = value_iter.next() {
        println!("Position: ({},{})", value.x, value.y)
    }
}

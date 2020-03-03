struct PositionComponent {
    x: f64,
    y: f64,
}

struct VelocityComponent {
    x: f64,
    y: f64,
}

pub fn main() {
    let mut positions = vec![
        PositionComponent { x: 0.0, y: 0.0 },
        PositionComponent { x: -42.0, y: -42.0 },
        PositionComponent { x: 234.0, y: 123.0 },
        PositionComponent { x: 6.0, y: 9.0 },
    ];
    let velocities = vec![
        VelocityComponent { x: 40.0, y: 10.0 },
        VelocityComponent { x: 30.0, y: 20.0 },
        VelocityComponent { x: 20.0, y: 30.0 },
        VelocityComponent { x: 10.0, y: 40.0 },
    ];

    let mut pos_iter = positions.iter_mut();
    let mut vel_iter = velocities.iter();

    while let (Some(pos), Some(vel)) = (pos_iter.next(), vel_iter.next()) {
        pos.x += vel.x;
        pos.y += vel.y;
    }

    let mut pos_iter = positions.iter();

    while let Some(position) = pos_iter.next() {
        println!("Position: ({},{})", position.x, position.y)
    }
}

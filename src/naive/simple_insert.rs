use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

struct World {
    transforms: Vec<Transform>,
    positions: Vec<Position>,
    rotations: Vec<Rotation>,
    velocities: Vec<Velocity>,
}

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World { 
            transforms: vec![],
            positions: vec![],
            rotations: vec![],
            velocities: vec![],
        };

        // Capture the actual insertion process, rather than initializing as-is
        world.transforms.extend(vec![Transform(Matrix4::from_scale(1.0)); 10000]);
        world.positions.extend(vec![Position(Vector3::unit_x()); 10000]);
        world.rotations.extend(vec![Rotation(Vector3::unit_x()); 10000]);
        world.velocities.extend(vec![Velocity(Vector3::unit_x()); 10000]);
    }
}

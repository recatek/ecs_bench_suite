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

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        Self(World {
            transforms: Vec::with_capacity(10000),
            positions: Vec::with_capacity(10000),
            rotations: Vec::with_capacity(10000),
            velocities: Vec::with_capacity(10000),
        })
    }

    pub fn run(&mut self) {
        self.0.transforms.extend(vec![Transform(Matrix4::from_scale(1.0)); 10000]);
        self.0.positions.extend(vec![Position(Vector3::unit_x()); 10000]);
        self.0.rotations.extend(vec![Rotation(Vector3::unit_x()); 10000]);
        self.0.velocities.extend(vec![Velocity(Vector3::unit_x()); 10000]);
    }
}

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
        for _ in 0..10000 {
            self.0.transforms.push(Transform(Matrix4::from_scale(1.0)));
        }
        for _ in 0..10000 {
            self.0.positions.push(Position(Vector3::unit_x()));
        }
        for _ in 0..10000 {
            self.0.rotations.push(Rotation(Vector3::unit_x()));
        }
        for _ in 0..10000 {
            self.0.velocities.push(Velocity(Vector3::unit_x()));
        }
    }
}

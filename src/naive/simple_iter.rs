use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

#[allow(dead_code)]
struct World {
    pub transforms: Vec<Transform>,
    pub positions: Vec<Position>,
    pub rotations: Vec<Rotation>,
    pub velocities: Vec<Velocity>,
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World { 
            transforms: vec![Transform(Matrix4::from_scale(1.0)); 10000],
            positions: vec![Position(Vector3::unit_x()); 10000],
            rotations: vec![Rotation(Vector3::unit_x()); 10000],
            velocities: vec![Velocity(Vector3::unit_x()); 10000],
        };

        Self(world)
    }

    pub fn run(&mut self) {
        for (velocity, position) in itertools::izip!(&self.0.velocities, &mut self.0.positions) {
            position.0 += velocity.0;
        }
    }
}

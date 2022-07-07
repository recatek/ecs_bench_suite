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
    transforms: Vec<Transform>,
    positions: Vec<Position>,
    rotations: Vec<Rotation>,
    velocities: Vec<Velocity>,
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World {
            transforms: vec![Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))); 1000],
            positions: vec![Position(Vector3::unit_x()); 1000],
            rotations: vec![Rotation(Vector3::unit_x()); 1000],
            velocities: vec![Velocity(Vector3::unit_x()); 1000],
        };

        Self(world)
    }

    pub fn run(&mut self) {
        for (position, transform) in itertools::izip!(&mut self.0.positions, &mut self.0.transforms) {
            for _ in 0..100 {
                transform.0 = transform.0.invert().unwrap();
            }

            position.0 = cgmath::Transform::transform_vector(&transform.0, position.0);
        }
    }
}

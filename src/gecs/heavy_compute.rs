use cgmath::*;
use gecs::prelude::*;

#[derive(Copy, Clone)]
pub struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
pub struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
pub struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
pub struct Velocity(Vector3<f32>);

ecs_world! {
    archetype!(Arch, 1000, Transform, Position, Rotation, Velocity);
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 0..1000 {
            world.push::<Arch>((
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            ));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        let world = &mut self.0;
        ecs_iter!(world, |position: &mut Position,
                          transform: &mut Transform| {
            for _ in 0..100 {
                transform.0 = transform.0.invert().unwrap();
            }

            position.0 = cgmath::Transform::transform_vector(&transform.0, position.0);
        });
    }
}

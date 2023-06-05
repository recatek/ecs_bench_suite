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
    ecs_name!(WorldSimpleIter);
    ecs_archetype!(Arch, 10000, Transform, Position, Rotation, Velocity);
}

pub struct Benchmark(WorldSimpleIter);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = WorldSimpleIter::default();

        for _ in 0..10000 {
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
        ecs_iter!(world, |velocity: &Velocity, position: &mut Position| {
            position.0 += velocity.0;
        });
    }
}

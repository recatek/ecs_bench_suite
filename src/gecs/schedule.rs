use gecs::prelude::*;

#[derive(Copy, Clone)]
pub struct A(f32);

#[derive(Copy, Clone)]
pub struct B(f32);

#[derive(Copy, Clone)]
pub struct C(f32);

#[derive(Copy, Clone)]
pub struct D(f32);

#[derive(Copy, Clone)]
pub struct E(f32);

ecs_world! {
    ecs_archetype!(Arch1, 10000, A, B);
    ecs_archetype!(Arch2, 10000, A, B, C);
    ecs_archetype!(Arch3, 10000, A, B, C, D);
    ecs_archetype!(Arch4, 10000, A, B, C, E);
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 0..10000 {
            world.push::<Arch1>((A(0.0), B(0.0)));
            world.push::<Arch2>((A(0.0), B(0.0), C(0.0)));
            world.push::<Arch3>((A(0.0), B(0.0), C(0.0), D(0.0)));
            world.push::<Arch4>((A(0.0), B(0.0), C(0.0), E(0.0)));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        let world = &mut self.0;

        ecs_iter!(world, |a: &mut A, b: &mut B| std::mem::swap(
            &mut a.0, &mut b.0
        ));

        ecs_iter!(world, |c: &mut C, d: &mut D| std::mem::swap(
            &mut c.0, &mut d.0
        ));

        ecs_iter!(world, |c: &mut C, e: &mut E| std::mem::swap(
            &mut c.0, &mut e.0
        ));
    }
}

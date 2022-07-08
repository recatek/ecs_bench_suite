use hecs::*;

#[derive(Copy, Clone)]
struct A(f32);

#[derive(Copy, Clone)]
struct B(f32);

#[derive(Copy, Clone)]
struct C(f32);

#[derive(Copy, Clone)]
struct D(f32);

#[derive(Copy, Clone)]
struct E(f32);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        Self(world)
    }

    pub fn run(&mut self) {
        for (_, (a, b)) in self.0.query_mut::<(&mut A, &mut B)>() {
            std::mem::swap(&mut a.0, &mut b.0);
        }
        for (_, (c, d)) in self.0.query_mut::<(&mut C, &mut D)>() {
            std::mem::swap(&mut c.0, &mut d.0);
        };
        for (_, (c, e)) in self.0.query_mut::<(&mut C, &mut E)>() {
            std::mem::swap(&mut c.0, &mut e.0);
        };
    }
}

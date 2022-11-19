use bevy_ecs::{prelude::*};

#[derive(Component)]
struct A(f32);
#[derive(Component)]
struct B(f32);
#[derive(Component)]
struct C(f32);
#[derive(Component)]
struct D(f32);
#[derive(Component)]
struct E(f32);

pub struct Benchmark<'w>(World, QueryState<(&'w mut A, &'w mut B)>, QueryState<(&'w mut C, &'w mut D)>, QueryState<(&'w mut C, &'w mut E)>);

impl<'w> Benchmark<'w> {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let query1 = world.query::<(&mut A, &mut B)>();
        let query2 = world.query::<(&mut C, &mut D)>();
        let query3 = world.query::<(&mut C, &mut E)>();

        Self(world, query1, query2, query3)
    }

    pub fn run(&mut self) {
        self.1.for_each_mut(&mut self.0, |(mut a, mut b)| {
            std::mem::swap(&mut a.0, &mut b.0);
        });
        self.2.for_each_mut(&mut self.0, |(mut c, mut d)| {
            std::mem::swap(&mut c.0, &mut d.0);
        });
        self.3.for_each_mut(&mut self.0, |(mut c, mut e)| {
            std::mem::swap(&mut c.0, &mut e.0);
        });
    }
}

use bevy_ecs::{prelude::*, schedule::Schedule};

#[derive(bevy_ecs::component::Component)]
struct A(f32);

#[derive(bevy_ecs::component::Component)]
struct B(f32);

#[derive(bevy_ecs::component::Component)]
struct C(f32);

#[derive(bevy_ecs::component::Component)]
struct D(f32);

#[derive(bevy_ecs::component::Component)]
struct E(f32);

fn ab(mut query: Query<(&mut A, &mut B)>) {
    query.for_each_mut(|(mut a, mut b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    });
}
fn cd(mut query: Query<(&mut C, &mut D)>) {
    query.for_each_mut(|(mut c, mut d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    });
}
fn ce(mut query: Query<(&mut C, &mut E)>) {
    query.for_each_mut(|(mut c, mut e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    });
}
pub struct Benchmark(World, Schedule);
impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));
        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let mut schedule = Schedule::default();
        schedule.add_stage("main", SystemStage::single_threaded());
        schedule.add_system_to_stage("main", ab);
        schedule.add_system_to_stage("main", cd);
        schedule.add_system_to_stage("main", ce);

        Self(world, schedule)
    }
    pub fn run(&mut self) {
        self.1.run(&mut self.0);
    }
}

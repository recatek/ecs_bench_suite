use bevy_ecs::prelude::*;
use bevy_tasks::TaskPool;
use cgmath::*;

#[derive(Component, Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Component, Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Component, Copy, Clone)]
struct Velocity(Vector3<f32>);

#[derive(Component, Copy, Clone)]
struct Transform(Matrix4<f32>);

pub struct Benchmark(World, Box<dyn System<In = (), Out = ()>>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..1000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        fn sys(task_pool: Res<TaskPool>, mut query: Query<(&mut Position, &mut Transform)>) {
            query.par_for_each_mut(&task_pool, 128, |(mut pos, mut trans)| {
                for _ in 0..100 {
                    trans.0 = trans.0.invert().unwrap();
                }

                pos.0 = cgmath::Transform::transform_vector(&trans.0, pos.0);
            });
        }

        world.insert_resource(TaskPool::default());
        let mut system = IntoSystem::into_system(sys);
        system.initialize(&mut world);
        system.update_archetype_component_access(&world);

        Self(world, Box::new(system))
    }

    pub fn run(&mut self) {
        self.1.run((), &mut self.0);
    }
}

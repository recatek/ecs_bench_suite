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

#[derive(Copy, Clone)]
pub struct F(f32);

#[derive(Copy, Clone)]
pub struct G(f32);

#[derive(Copy, Clone)]
pub struct H(f32);

#[derive(Copy, Clone)]
pub struct I(f32);

#[derive(Copy, Clone)]
pub struct J(f32);

#[derive(Copy, Clone)]
pub struct K(f32);

#[derive(Copy, Clone)]
pub struct L(f32);

#[derive(Copy, Clone)]
pub struct M(f32);

#[derive(Copy, Clone)]
pub struct N(f32);

#[derive(Copy, Clone)]
pub struct O(f32);

#[derive(Copy, Clone)]
pub struct P(f32);

#[derive(Copy, Clone)]
pub struct Q(f32);

#[derive(Copy, Clone)]
pub struct R(f32);

#[derive(Copy, Clone)]
pub struct S(f32);

#[derive(Copy, Clone)]
pub struct T(f32);

#[derive(Copy, Clone)]
pub struct U(f32);

#[derive(Copy, Clone)]
pub struct V(f32);

#[derive(Copy, Clone)]
pub struct W(f32);

#[derive(Copy, Clone)]
pub struct X(f32);

#[derive(Copy, Clone)]
pub struct Y(f32);

#[derive(Copy, Clone)]
pub struct Z(f32);

#[derive(Copy, Clone)]
pub struct Data(f32);

ecs_world! {
    ecs_archetype!(ArchA, 20, A, Data);
    ecs_archetype!(ArchB, 20, B, Data);
    ecs_archetype!(ArchC, 20, C, Data);
    ecs_archetype!(ArchD, 20, D, Data);
    ecs_archetype!(ArchE, 20, E, Data);
    ecs_archetype!(ArchF, 20, F, Data);
    ecs_archetype!(ArchG, 20, G, Data);
    ecs_archetype!(ArchH, 20, H, Data);
    ecs_archetype!(ArchI, 20, I, Data);
    ecs_archetype!(ArchJ, 20, J, Data);
    ecs_archetype!(ArchK, 20, K, Data);
    ecs_archetype!(ArchL, 20, L, Data);
    ecs_archetype!(ArchM, 20, M, Data);
    ecs_archetype!(ArchN, 20, N, Data);
    ecs_archetype!(ArchO, 20, O, Data);
    ecs_archetype!(ArchP, 20, P, Data);
    ecs_archetype!(ArchQ, 20, Q, Data);
    ecs_archetype!(ArchR, 20, R, Data);
    ecs_archetype!(ArchS, 20, S, Data);
    ecs_archetype!(ArchT, 20, T, Data);
    ecs_archetype!(ArchU, 20, U, Data);
    ecs_archetype!(ArchV, 20, V, Data);
    ecs_archetype!(ArchW, 20, W, Data);
    ecs_archetype!(ArchX, 20, X, Data);
    ecs_archetype!(ArchY, 20, Y, Data);
    ecs_archetype!(ArchZ, 20, Z, Data);
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 0..20 {
            world.push::<ArchA>((A(0.0), Data(1.0)));
            world.push::<ArchB>((B(0.0), Data(1.0)));
            world.push::<ArchC>((C(0.0), Data(1.0)));
            world.push::<ArchD>((D(0.0), Data(1.0)));
            world.push::<ArchE>((E(0.0), Data(1.0)));
            world.push::<ArchF>((F(0.0), Data(1.0)));
            world.push::<ArchG>((G(0.0), Data(1.0)));
            world.push::<ArchH>((H(0.0), Data(1.0)));
            world.push::<ArchI>((I(0.0), Data(1.0)));
            world.push::<ArchJ>((J(0.0), Data(1.0)));
            world.push::<ArchK>((K(0.0), Data(1.0)));
            world.push::<ArchL>((L(0.0), Data(1.0)));
            world.push::<ArchM>((M(0.0), Data(1.0)));
            world.push::<ArchN>((N(0.0), Data(1.0)));
            world.push::<ArchO>((O(0.0), Data(1.0)));
            world.push::<ArchP>((P(0.0), Data(1.0)));
            world.push::<ArchQ>((Q(0.0), Data(1.0)));
            world.push::<ArchR>((R(0.0), Data(1.0)));
            world.push::<ArchS>((S(0.0), Data(1.0)));
            world.push::<ArchT>((T(0.0), Data(1.0)));
            world.push::<ArchU>((U(0.0), Data(1.0)));
            world.push::<ArchV>((V(0.0), Data(1.0)));
            world.push::<ArchW>((W(0.0), Data(1.0)));
            world.push::<ArchX>((X(0.0), Data(1.0)));
            world.push::<ArchY>((Y(0.0), Data(1.0)));
            world.push::<ArchZ>((Z(0.0), Data(1.0)));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        let world = &mut self.0;
        ecs_iter!(world, |d: &mut Data| d.0 *= 2.0);
    }
}

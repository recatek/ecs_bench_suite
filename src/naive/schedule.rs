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

#[allow(dead_code)]
struct World {
    arch1_a: Vec<A>,
    arch1_b: Vec<B>,

    arch2_a: Vec<A>,
    arch2_b: Vec<B>,
    arch2_c: Vec<C>,

    arch3_a: Vec<A>,
    arch3_b: Vec<B>,
    arch3_c: Vec<C>,
    arch3_d: Vec<D>,

    arch4_a: Vec<A>,
    arch4_b: Vec<B>,
    arch4_c: Vec<C>,
    arch4_e: Vec<E>,
}

impl World {
    fn query_ab(&mut self) {
        for (a, b) in itertools::izip!(&mut self.arch1_a, &mut self.arch1_b) {
            std::mem::swap(&mut a.0, &mut b.0);
        }
        for (a, b) in itertools::izip!(&mut self.arch2_a, &mut self.arch2_b) {
            std::mem::swap(&mut a.0, &mut b.0);
        }
        for (a, b) in itertools::izip!(&mut self.arch3_a, &mut self.arch3_b) {
            std::mem::swap(&mut a.0, &mut b.0);
        }
        for (a, b) in itertools::izip!(&mut self.arch4_a, &mut self.arch4_b) {
            std::mem::swap(&mut a.0, &mut b.0);
        }
    }

    fn query_cd(&mut self) {
        for (c, d) in itertools::izip!(&mut self.arch3_c, &mut self.arch3_d) {
            std::mem::swap(&mut c.0, &mut d.0);
        }
    }

    fn query_ce(&mut self) {
        for (c, e) in itertools::izip!(&mut self.arch4_c, &mut self.arch4_e) {
            std::mem::swap(&mut c.0, &mut e.0);
        }
    }
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World {
            arch1_a: vec![A(0.0); 10000],
            arch1_b: vec![B(0.0); 10000],
        
            arch2_a: vec![A(0.0); 10000],
            arch2_b: vec![B(0.0); 10000],
            arch2_c: vec![C(0.0); 10000],
        
            arch3_a: vec![A(0.0); 10000],
            arch3_b: vec![B(0.0); 10000],
            arch3_c: vec![C(0.0); 10000],
            arch3_d: vec![D(0.0); 10000],
        
            arch4_a: vec![A(0.0); 10000],
            arch4_b: vec![B(0.0); 10000],
            arch4_c: vec![C(0.0); 10000],
            arch4_e: vec![E(0.0); 10000],
        };

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.query_ab();
        self.0.query_cd();
        self.0.query_ce();
    }
}

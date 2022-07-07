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

#[derive(Copy, Clone)]
struct F(f32);

#[derive(Copy, Clone)]
struct G(f32);

#[derive(Copy, Clone)]
struct H(f32);

#[derive(Copy, Clone)]
struct I(f32);

#[derive(Copy, Clone)]
struct J(f32);

#[derive(Copy, Clone)]
struct K(f32);

#[derive(Copy, Clone)]
struct L(f32);

#[derive(Copy, Clone)]
struct M(f32);

#[derive(Copy, Clone)]
struct N(f32);

#[derive(Copy, Clone)]
struct O(f32);

#[derive(Copy, Clone)]
struct P(f32);

#[derive(Copy, Clone)]
struct Q(f32);

#[derive(Copy, Clone)]
struct R(f32);

#[derive(Copy, Clone)]
struct S(f32);

#[derive(Copy, Clone)]
struct T(f32);

#[derive(Copy, Clone)]
struct U(f32);

#[derive(Copy, Clone)]
struct V(f32);

#[derive(Copy, Clone)]
struct W(f32);

#[derive(Copy, Clone)]
struct X(f32);

#[derive(Copy, Clone)]
struct Y(f32);

#[derive(Copy, Clone)]
struct Z(f32);

#[derive(Copy, Clone)]
struct Data(f32);

#[allow(dead_code)]
struct World {
    a_self: Vec<A>,
    a_data: Vec<Data>,

    b_self: Vec<B>,
    b_data: Vec<Data>,

    c_self: Vec<C>,
    c_data: Vec<Data>,

    d_self: Vec<D>,
    d_data: Vec<Data>,

    e_self: Vec<E>,
    e_data: Vec<Data>,

    f_self: Vec<F>,
    f_data: Vec<Data>,

    g_self: Vec<G>,
    g_data: Vec<Data>,

    h_self: Vec<H>,
    h_data: Vec<Data>,

    i_self: Vec<I>,
    i_data: Vec<Data>,

    j_self: Vec<J>,
    j_data: Vec<Data>,

    k_self: Vec<K>,
    k_data: Vec<Data>,

    l_self: Vec<L>,
    l_data: Vec<Data>,

    m_self: Vec<M>,
    m_data: Vec<Data>,

    n_self: Vec<N>,
    n_data: Vec<Data>,

    o_self: Vec<O>,
    o_data: Vec<Data>,

    p_self: Vec<P>,
    p_data: Vec<Data>,

    q_self: Vec<Q>,
    q_data: Vec<Data>,

    r_self: Vec<R>,
    r_data: Vec<Data>,

    s_self: Vec<S>,
    s_data: Vec<Data>,

    t_self: Vec<T>,
    t_data: Vec<Data>,

    u_self: Vec<U>,
    u_data: Vec<Data>,

    v_self: Vec<V>,
    v_data: Vec<Data>,

    w_self: Vec<W>,
    w_data: Vec<Data>,

    x_self: Vec<X>,
    x_data: Vec<Data>,

    y_self: Vec<Y>,
    y_data: Vec<Data>,

    z_self: Vec<Z>,
    z_data: Vec<Data>,
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World {
            a_self: vec![A(0.0); 20],
            a_data: vec![Data(1.0); 20],

            b_self: vec![B(0.0); 20],
            b_data: vec![Data(1.0); 20],

            c_self: vec![C(0.0); 20],
            c_data: vec![Data(1.0); 20],

            d_self: vec![D(0.0); 20],
            d_data: vec![Data(1.0); 20],

            e_self: vec![E(0.0); 20],
            e_data: vec![Data(1.0); 20],

            f_self: vec![F(0.0); 20],
            f_data: vec![Data(1.0); 20],

            g_self: vec![G(0.0); 20],
            g_data: vec![Data(1.0); 20],

            h_self: vec![H(0.0); 20],
            h_data: vec![Data(1.0); 20],

            i_self: vec![I(0.0); 20],
            i_data: vec![Data(1.0); 20],

            j_self: vec![J(0.0); 20],
            j_data: vec![Data(1.0); 20],

            k_self: vec![K(0.0); 20],
            k_data: vec![Data(1.0); 20],

            l_self: vec![L(0.0); 20],
            l_data: vec![Data(1.0); 20],

            m_self: vec![M(0.0); 20],
            m_data: vec![Data(1.0); 20],

            n_self: vec![N(0.0); 20],
            n_data: vec![Data(1.0); 20],

            o_self: vec![O(0.0); 20],
            o_data: vec![Data(1.0); 20],

            p_self: vec![P(0.0); 20],
            p_data: vec![Data(1.0); 20],

            q_self: vec![Q(0.0); 20],
            q_data: vec![Data(1.0); 20],

            r_self: vec![R(0.0); 20],
            r_data: vec![Data(1.0); 20],

            s_self: vec![S(0.0); 20],
            s_data: vec![Data(1.0); 20],

            t_self: vec![T(0.0); 20],
            t_data: vec![Data(1.0); 20],

            u_self: vec![U(0.0); 20],
            u_data: vec![Data(1.0); 20],

            v_self: vec![V(0.0); 20],
            v_data: vec![Data(1.0); 20],

            w_self: vec![W(0.0); 20],
            w_data: vec![Data(1.0); 20],

            x_self: vec![X(0.0); 20],
            x_data: vec![Data(1.0); 20],

            y_self: vec![Y(0.0); 20],
            y_data: vec![Data(1.0); 20],

            z_self: vec![Z(0.0); 20],
            z_data: vec![Data(1.0); 20],
        };

        Self(world)
    }

    pub fn run(&mut self) {
        for data in &mut self.0.a_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.b_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.c_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.d_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.e_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.f_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.g_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.h_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.i_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.j_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.k_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.l_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.m_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.n_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.o_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.p_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.q_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.r_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.s_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.t_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.u_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.v_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.w_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.x_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.y_data {
            data.0 *= 2.0;
        }
        for data in &mut self.0.z_data {
            data.0 *= 2.0;
        }
    }
}

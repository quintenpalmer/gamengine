use tylar;

use std::fmt;
use std::marker::PhantomData;

use tylar::{Succ, Zero, Pred};

pub trait TypeLevelZero {}
pub trait TypeLevelAdd<RHS> {
    type Out;
}
pub trait TypeLevelSub<RHS> {
    type Out;
}
pub trait TypeLevelMul<RHS> {
    type Out;
}
pub trait TypeLevelDiv<RHS> {
    type Out;
}
pub trait TypeLevelDisplay {
    fn fmt(&mut fmt::Formatter) -> fmt::Result;
}

#[derive(Copy, Clone, Debug)]
pub struct SIUnit<M, S, G, A, K, O, C>
    where M: tylar::NumType,
          S: tylar::NumType,
          G: tylar::NumType,
          A: tylar::NumType,
          K: tylar::NumType,
          O: tylar::NumType,
          C: tylar::NumType
{
    meter: PhantomData<M>,
    second: PhantomData<S>,
    gram: PhantomData<G>,
    ampere: PhantomData<A>,
    kelvin: PhantomData<K>,
    mole: PhantomData<O>,
    candela: PhantomData<C>,
}

impl<M, S, G, A, K, O, C> TypeLevelDisplay for SIUnit<M, S, G, A, K, O, C>
    where M: tylar::NumType,
          S: tylar::NumType,
          G: tylar::NumType,
          A: tylar::NumType,
          K: tylar::NumType,
          O: tylar::NumType,
          C: tylar::NumType
{
    fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let m: i8 = M::new().into();
        let s: i8 = S::new().into();
        let g: i8 = G::new().into();
        let a: i8 = A::new().into();
        let k: i8 = K::new().into();
        let mol: i8 = O::new().into();
        let cd: i8 = C::new().into();
        let mut tops = vec![];
        let mut bottoms = vec![];
        for &(v, i) in [("m", m),
                        ("s", s),
                        ("g", g),
                        ("a", a),
                        ("K", k),
                        ("mol", mol),
                        ("cd", cd)]
            .iter() {
            if i > 0 {
                match get_unit_display(String::from(v), i) {
                    Some(t) => tops.push(t),
                    None => (),
                };
            } else {
                match get_unit_display(String::from(v), i.abs()) {
                    Some(b) => bottoms.push(b),
                    None => (),
                };
            }
        }
        for t in tops.iter() {
            try!(write!(f, "{}", t));
        }
        try!(write!(f, "/"));
        for b in bottoms.iter() {
            try!(write!(f, "{}", b));
        }
        return Ok(());
    }
}

fn get_unit_display(v: String, i: i8) -> Option<String> {
    match i {
        0 => None,
        1 => Some(format!("{}", v)),
        _ => Some(format!("{}^{}", v, i)),
    }
}

impl<M, S, G, A, K, O, C> TypeLevelAdd<SIUnit<M, S, G, A, K, O, C>> for SIUnit<M, S, G, A, K, O, C>
    where M: tylar::NumType,
          S: tylar::NumType,
          G: tylar::NumType,
          A: tylar::NumType,
          K: tylar::NumType,
          O: tylar::NumType,
          C: tylar::NumType
{
    type Out = SIUnit<M, S, G, A, K, O, C>;
}

impl<M, S, G, A, K, O, C> TypeLevelSub<SIUnit<M, S, G, A, K, O, C>> for SIUnit<M, S, G, A, K, O, C>
    where M: tylar::NumType,
          S: tylar::NumType,
          G: tylar::NumType,
          A: tylar::NumType,
          K: tylar::NumType,
          O: tylar::NumType,
          C: tylar::NumType
{
    type Out = SIUnit<M, S, G, A, K, O, C>;
}

impl<M1, S1, G1, A1, K1, O1, C1, M2, S2, G2, A2, K2, O2, C2> TypeLevelMul<SIUnit<M2,
                                                                                 S2,
                                                                                 G2,
                                                                                 A2,
                                                                                 K2,
                                                                                 O2,
                                                                                 C2>>
    for SIUnit<M1, S1, G1, A1, K1, O1, C1>
    where M1: tylar::NumType + tylar::Add<M2>,
          S1: tylar::NumType + tylar::Add<S2>,
          G1: tylar::NumType + tylar::Add<G2>,
          A1: tylar::NumType + tylar::Add<A2>,
          K1: tylar::NumType + tylar::Add<K2>,
          O1: tylar::NumType + tylar::Add<O2>,
          C1: tylar::NumType + tylar::Add<C2>,
          M2: tylar::NumType,
          S2: tylar::NumType,
          G2: tylar::NumType,
          A2: tylar::NumType,
          K2: tylar::NumType,
          O2: tylar::NumType,
          C2: tylar::NumType
{
    type Out = SIUnit<M1::Out, S1::Out, G1::Out, A1::Out, K1::Out, O1::Out, C1::Out>;
}

impl<M1, S1, G1, A1, K1, O1, C1, M2, S2, G2, A2, K2, O2, C2> TypeLevelDiv<SIUnit<M2,
                                                                                 S2,
                                                                                 G2,
                                                                                 A2,
                                                                                 K2,
                                                                                 O2,
                                                                                 C2>>
    for SIUnit<M1, S1, G1, A1, K1, O1, C1>
    where M1: tylar::NumType + tylar::Sub<M2>,
          S1: tylar::NumType + tylar::Sub<S2>,
          G1: tylar::NumType + tylar::Sub<G2>,
          A1: tylar::NumType + tylar::Sub<A2>,
          K1: tylar::NumType + tylar::Sub<K2>,
          O1: tylar::NumType + tylar::Sub<O2>,
          C1: tylar::NumType + tylar::Sub<C2>,
          M2: tylar::NumType,
          S2: tylar::NumType,
          G2: tylar::NumType,
          A2: tylar::NumType,
          K2: tylar::NumType,
          O2: tylar::NumType,
          C2: tylar::NumType
{
    type Out = SIUnit<M1::Out, S1::Out, G1::Out, A1::Out, K1::Out, O1::Out, C1::Out>;
}

pub fn constant() -> SIUnit<Zero, Zero, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn meters() -> SIUnit<Succ<Zero>, Zero, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn hertz() -> SIUnit<Zero, Pred<Zero>, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn seconds() -> SIUnit<Zero, Succ<Zero>, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn grams() -> SIUnit<Zero, Zero, Succ<Zero>, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn velocity() -> SIUnit<Succ<Zero>, Pred<Zero>, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

pub fn acceleration() -> SIUnit<Succ<Zero>, Pred<Pred<Zero>>, Zero, Zero, Zero, Zero, Zero> {
    SIUnit {
        meter: PhantomData,
        second: PhantomData,
        gram: PhantomData,
        ampere: PhantomData,
        kelvin: PhantomData,
        mole: PhantomData,
        candela: PhantomData,
    }
}

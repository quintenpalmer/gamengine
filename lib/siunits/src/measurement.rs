use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use num::Num;

use tylar::{Succ, Zero, Pred};

use units::{SIUnit, TypeLevelAdd, TypeLevelSub, TypeLevelMul, TypeLevelDiv, TypeLevelDisplay};

#[derive(Copy, Clone, Debug)]
pub struct Measurement<U, N: Num> {
    unit: PhantomData<U>,
    magnitude: N,
}

impl<U, N> fmt::Display for Measurement<U, N>
    where U: TypeLevelDisplay,
          N: fmt::Display + Num
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "'{} (", self.magnitude));
        try!(U::fmt(f));
        try!(write!(f, ")'"));
        Ok(())
    }
}

impl<U, N: Num> Measurement<U, N> {
    fn new(n: N) -> Self {
        Measurement {
            unit: PhantomData,
            magnitude: n,
        }
    }
}

pub fn constant<N: Num>(n: N) -> Measurement<SIUnit<Zero, Zero, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn meters<N: Num>(n: N)
                      -> Measurement<SIUnit<Succ<Zero>, Zero, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn hertz<N: Num>(n: N)
                     -> Measurement<SIUnit<Zero, Pred<Zero>, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn seconds<N: Num>
    (n: N)
     -> Measurement<SIUnit<Zero, Succ<Zero>, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn grams<N: Num>(n: N)
                     -> Measurement<SIUnit<Zero, Zero, Succ<Zero>, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn velocity<N: Num>
    (n: N)
     -> Measurement<SIUnit<Succ<Zero>, Pred<Zero>, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

pub fn acceleration<N: Num>
    (n: N)
     -> Measurement<SIUnit<Succ<Zero>, Pred<Pred<Zero>>, Zero, Zero, Zero, Zero, Zero>, N> {
    return {
        Measurement::new(n)
    };
}

impl<U1, U2, N> Add<Measurement<U2, N>> for Measurement<U1, N>
    where U1: TypeLevelAdd<U2>,
          N: Num
{
    type Output = Measurement<U1::Out, N>;

    fn add(self, rhs: Measurement<U2, N>) -> Self::Output {
        Measurement::new(self.magnitude + rhs.magnitude)
    }
}

impl<U1, U2, N> Sub<Measurement<U2, N>> for Measurement<U1, N>
    where U1: TypeLevelSub<U2>,
          N: Num
{
    type Output = Measurement<U1::Out, N>;

    fn sub(self, rhs: Measurement<U2, N>) -> Self::Output {
        Measurement::new(self.magnitude - rhs.magnitude)
    }
}

impl<U1, U2, N> Mul<Measurement<U2, N>> for Measurement<U1, N>
    where U1: TypeLevelMul<U2>,
          N: Num
{
    type Output = Measurement<U1::Out, N>;

    fn mul(self, rhs: Measurement<U2, N>) -> Self::Output {
        Measurement::new(self.magnitude * rhs.magnitude)
    }
}

impl<U1, U2, N> Div<Measurement<U2, N>> for Measurement<U1, N>
    where U1: TypeLevelDiv<U2>,
          N: Num
{
    type Output = Measurement<U1::Out, N>;

    fn div(self, rhs: Measurement<U2, N>) -> Self::Output {
        Measurement::new(self.magnitude / rhs.magnitude)
    }
}

use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quant(pub u32);

impl Add for Quant {
    type Output = Quant;

    fn add(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this + other)
    }
}

impl Mul for Quant {
    type Output = Quant;

    fn mul(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this * other)
    }
}

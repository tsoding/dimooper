use std::ops::{Add, Mul, Sub, Rem, Div};

use serde::{Serialize, Serializer, Deserialize, Deserializer};

// FIXME(#125): Autoderive arithmetic operations for Quant
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Quant(pub u32);

impl Serialize for Quant {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // let &Quant(this) = self;
        // s.emit_u32(this)
    }
}

impl<'de> Deserialize<'de> for Quant {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        unimplemented!()
        // TODO: reimplement with serde
        // d.read_u32().map(|q| Quant(q))
    }
}

impl Add for Quant {
    type Output = Quant;

    fn add(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this + other)
    }
}

impl Sub for Quant {
    type Output = Quant;

    fn sub(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this - other)
    }
}

impl Mul for Quant {
    type Output = Quant;

    fn mul(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this * other)
    }
}

impl Div for Quant {
    type Output = Quant;

    fn div(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this / other)
    }
}

impl Rem for Quant {
    type Output = Quant;

    fn rem(self, Quant(other): Quant) -> Quant {
        let Quant(this) = self;
        Quant(this % other)
    }
}

#[cfg(test)]
mod tests {
    use super::Quant;
    use serde_json;

    #[test]
    fn test_quant_serialization() {
        let q = Quant(42);

        assert_eq!(Quant(42),
                   serde_json::from_str(&serde_json::to_string(&q).unwrap()).unwrap())
    }
}

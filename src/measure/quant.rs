use std::ops::{Add, Mul, Sub, Rem, Div};

// FIXME(#125): Autoderive arithmetic operations for Quant
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quant(pub u32);

impl Quant {
    pub fn as_u32(&self) -> u32 {
        let Quant(this) = *self;
        this
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

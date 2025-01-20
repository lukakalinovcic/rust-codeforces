use crate::modulo::utils::{add, div, inverse, mul, power, sub};

use super::utils::gen_inverses;

#[derive(Clone, Copy, Default)]
pub struct ModInt<const M: u32> {
    x: u32, // always in 0..M
}

impl<const M: u32> std::fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const M: u32> std::fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = if self.x > M / 2 {
            self.x as i32 - M as i32
        } else {
            self.x as i32
        };
        if f.alternate() {
            write!(f, "{x} (mod {M})")
        } else {
            write!(f, "{x}")
        }
    }
}

impl<const M: u32> From<i32> for ModInt<M> {
    fn from(x: i32) -> Self {
        let x = if x < 0 {
            (x % M as i32 + M as i32) % M as i32
        } else if x >= M as i32 {
            x % M as i32
        } else {
            x
        } as u32;
        Self { x }
    }
}

impl<const M: u32> From<u32> for ModInt<M> {
    fn from(x: u32) -> Self {
        Self {
            x: if x >= M { x % M } else { x },
        }
    }
}

impl<const M: u32> From<usize> for ModInt<M> {
    fn from(x: usize) -> Self {
        Self {
            x: (x % M as usize) as u32,
        }
    }
}

impl<const M: u32> core::ops::Add for ModInt<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: add::<M>(self.x, other.x),
        }
    }
}

impl<const M: u32> core::ops::Sub for ModInt<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: sub::<M>(self.x, other.x),
        }
    }
}

impl<const M: u32> core::ops::Mul for ModInt<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: mul::<M>(self.x, other.x),
        }
    }
}

impl<const M: u32> core::ops::Div for ModInt<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: div::<M>(self.x, other.x),
        }
    }
}

impl<const M: u32> ModInt<M> {
    pub fn power(&self, n: u32) -> Self {
        Self {
            x: power::<M>(self.x, n),
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            x: inverse::<M>(self.x),
        }
    }

    pub fn unwrap(self) -> u32 {
        self.x
    }

    pub fn get(&self) -> u32 {
        self.x
    }

    pub fn gen_inverses(n: usize) -> Vec<Self> {
        gen_inverses::<M>(n)
            .into_iter()
            .map(|x| Self::from(x))
            .collect()
    }
}

impl<const M: u32> core::ops::AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        self.x = add::<M>(self.x, other.x);
    }
}

impl<const M: u32> core::ops::SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        self.x = sub::<M>(self.x, other.x);
    }
}

impl<const M: u32> core::ops::MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        self.x = mul::<M>(self.x, other.x);
    }
}

impl<const M: u32> core::ops::DivAssign for ModInt<M> {
    fn div_assign(&mut self, other: Self) {
        self.x = div::<M>(self.x, other.x);
    }
}

impl<const M: u32> core::ops::Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: sub::<M>(0, self.x),
        }
    }
}

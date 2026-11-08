//TASK(20261107-225625-620-n6-219): add serde flag

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct BitMask<const N: usize>(pub [u128; N]);

impl<const N: usize> Default for BitMask<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> std::ops::BitAnd for BitMask<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] & rhs.0[i]))
    }
}

impl<const N: usize> std::ops::BitOr for BitMask<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] | rhs.0[i]))
    }
}

impl<const N: usize> std::ops::BitXor for BitMask<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] ^ rhs.0[i]))
    }
}

impl<const N: usize> std::ops::Sub for BitMask<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] & !rhs.0[i]))
    }
}

impl<const N: usize> std::ops::Not for BitMask<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self(std::array::from_fn(|i| !self.0[i]))
    }
}

impl<const N: usize> std::ops::BitAndAssign for BitMask<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] &= rhs.0[i];
        }
    }
}

impl<const N: usize> std::ops::BitOrAssign for BitMask<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] |= rhs.0[i];
        }
    }
}

impl<const N: usize> std::ops::BitXorAssign for BitMask<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] ^= rhs.0[i];
        }
    }
}

impl<const N: usize> std::ops::SubAssign for BitMask<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] &= !rhs.0[i];
        }
    }
}

impl<const N: usize> BitMask<N> {
    pub fn count_ones(self) -> usize {
        self.0.iter().map(|word| word.count_ones() as usize).sum()
    }
}

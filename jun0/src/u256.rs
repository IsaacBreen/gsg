use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, Eq)]
pub struct u256(pub(crate) [u64; 4]);

impl u256 {
    pub const fn zero() -> Self {
        u256([0, 0, 0, 0])
    }

    pub const fn one() -> Self {
        u256([1, 0, 0, 0])
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    pub fn overflow_add(a: u64, b: u64, carry: &mut bool) -> u64 {
        let (res1, overflow1) = a.overflowing_add(b);
        let (res2, overflow2) = res1.overflowing_add(*carry as u64);
        *carry = overflow1 || overflow2;
        res2
    }

    pub fn overflow_sub(a: u64, b: u64, borrow: &mut bool) -> u64 {
        let (res1, overflow1) = a.overflowing_sub(b);
        let (res2, overflow2) = res1.overflowing_sub(*borrow as u64);
        *borrow = overflow1 || overflow2;
        res2
    }

    pub fn overflowing_add(self, other: u256) -> (u256, bool) {
        let mut carry = false;
        let mut res = [0u64; 4];

        for i in 0..4 {
            res[i] = Self::overflow_add(self.0[i], other.0[i], &mut carry);
        }

        (u256(res), carry)
    }

    pub fn overflowing_sub(self, other: u256) -> (u256, bool) {
        let mut borrow = false;
        let mut res = [0u64; 4];

        for i in 0..4 {
            res[i] = Self::overflow_sub(self.0[i], other.0[i], &mut borrow);
        }

        (u256(res), borrow)
    }
}

// Bitwise ops
impl u256 {
    pub fn is_set(&self, index: usize) -> bool {
        self.0[index / 64] & (1 << (index % 64)) != 0
    }

    pub fn set_bit(&mut self, index: usize) {
        let x: u64 = 1 << (index % 64);
        self.0[index / 64] |= x;
    }

    pub fn clear_bit(&mut self, index: usize) {
        let x: u64 = 1 << (index % 64);
        self.0[index / 64] &= !x;
    }
}

// Implement Add for U256
impl Add for u256 {
    type Output = u256;

    fn add(self, other: u256) -> u256 {
        self.overflowing_add(other).0
    }
}

// Implement Sub for U256
impl Sub for u256 {
    type Output = u256;

    fn sub(self, other: u256) -> u256 {
        self.overflowing_sub(other).0
    }
}

// Implement PartialEq for U256
impl PartialEq for u256 {
    fn eq(&self, other: &u256) -> bool {
        self.0 == other.0
    }
}

// Implement PartialOrd for U256
impl PartialOrd for u256 {
    fn partial_cmp(&self, other: &u256) -> Option<Ordering> {
        for i in (0..4).rev() {
            match self.0[i].cmp(&other.0[i]) {
                Ordering::Equal => continue,
                other => return Some(other),
            }
        }
        Some(Ordering::Equal)
    }
}

// Implement Display for U256
impl fmt::Display for u256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}{:016x}{:016x}{:016x}", self.0[3], self.0[2], self.0[1], self.0[0])
    }
}

fn main() {
    let a = u256([1, 0, 0, 0]);
    let b = u256([2, 0, 0, 0]);

    let c = a + b;
    let d = b - a;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
//! # Const-time-bignum
//! A bignum library that operates in constant time and without any heap allocations.
//!
//! ⚠️ This library is currently under development and should not be used.


use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct u288([u8; 36]); // 288 bit unsigned integer (8x36)

impl fmt::Display for u288 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Add for u288 {
    type Output = u288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = self;
        let mut carry = 0;
        for (i, byte) in output.0.iter_mut().enumerate() {
            // LSB first
            let sum: u64 = *byte as u64 + other.0[i] as u64 + carry as u64;
            *byte = (sum % 256) as u8;
            carry = sum / 256;
        }
        if carry > 0 {
            panic!("overflow");
        }
        output
    }
}

impl Sub for u288 {
    type Output = u288;
    fn sub(self, other: Self) -> Self::Output {
        let mut output = self;
        let mut carry = 0;
        for (i, byte) in output.0.iter_mut().enumerate() {
            let difference: i64 = *byte as i64 - other.0[i] as i64 - carry as i64;
            *byte = ((difference + 256) % 256) as u8;
            carry = difference.is_negative() as u8;
        }
        if carry > 0 {
            panic!("overflow");
        }
        output
    }
}

impl Mul for u288 {
    type Output = u288;
    fn mul(self, other: Self) -> Self::Output {
        let mut total_sum = u288::new();
        for (i, byte_self) in self.0.iter().enumerate() {
            // Multiply entire second number by each byte in self
            let mut working_sum = other;
            let mut carry = 0;
            for byte_other in working_sum.0.iter_mut() {
                let product = *byte_other as u64 * *byte_self as u64 + carry as u64;
                *byte_other = (product % 256) as u8;
                carry = product / 256;
            }
            if carry > 0 {
                panic!("overflow");
            }
            working_sum.0.rotate_right(i);
            total_sum = total_sum + working_sum;
        }
        total_sum
    }
}

// NOTE: This shifts in base 256
impl Shl<u288> for u288 {
    type Output = u288;
    fn shl(mut self, other: Self) -> Self::Output {
        let mut output = self;
        let mut i = u288::new(); // initializes to 0
        let one = u288::from_hex("1");
        while other > i {
            for j in 0..self.0.len() - 1 {
                output.0[j + 1] = self.0[j];
            }
            output.0[0] = 0;
            self = output;
            i = i + one; // Increment
        }
        output
    }
}

// NOTE: This shifts in base 256
impl Shl<usize> for u288 {
    type Output = u288;
    fn shl(mut self, other: usize) -> Self::Output {
        let mut output = self;
        let mut i: usize = 0; // initializes to 0
        while other > i {
            for j in 0..self.0.len() - 1 {
                output.0[j + 1] = self.0[j];
            }
            output.0[0] = 0;
            self = output;
            i += 1; // Increment
        }
        output
    }
}

// NOTE: This shifts in base 256
impl Shr<u288> for u288 {
    type Output = u288;
    fn shr(mut self, other: Self) -> Self::Output {
        let mut output = self;
        let mut i = u288::new(); // initializes to 0
        let one = u288::from_hex("1");
        while other > i {
            for j in (1..self.0.len() - 2).rev() {
                output.0[j - 1] = self.0[j];
            }
            output.0[output.0.len() - 1] = 0;
            self = output;
            i = i + one; // Increment
        }
        output
    }
}

// NOTE: This shifts in base 256
impl Shr<usize> for u288 {
    type Output = u288;
    fn shr(mut self, other: usize) -> Self::Output {
        let mut output = self;
        let mut i: usize = 0; // initializes to 0
        while other > i {
            for j in (1..self.0.len() - 2).rev() {
                output.0[j - 1] = self.0[j];
            }
            output.0[output.0.len() - 1] = 0;
            self = output;
            i += 1; // Increment
        }
        output
    }
}

// This is slow. TODO: Look into implementing a more performant algorithm!
// TODO: Do this in constant time!
impl Rem for u288 {
    type Output = u288;
    fn rem(self, other: Self) -> Self::Output {
        let mut numerator = self;
        let mut divisor = other;
        let mut quotient = u288::new(); // 0
        let one = u288::from_hex("1");

        // Align divisor to msb of numerator and store the shift amount in n
        let mut n: usize = 0;
        let mut flag = 1; // Flag to detect when msb has been hit
        for i in (0..numerator.0.len()).rev() {
            // Iterate over the bytes backwards
            n += flag & (numerator.0[i] != 0 && divisor.0[i] == 0) as usize;
            flag &= (divisor.0[i] == 0) as usize;
        }
        divisor = divisor << n; // TODO: Make this constant time!

        // TODO: This is temporary! Need to find a more permament solution
        let mut n: i64 = n as i64;

        // Keep shifting divisor to the right (decrease, in-memory left shift due to le)
        while other <= numerator {
            // Subtract until not possible anymore, then add to quotient
            let mut i = u288::new();
            while divisor <= numerator {
                numerator = numerator - divisor;
                i = i + one;
            }
            quotient = quotient + i << n as usize;
            n -= 1;
            divisor = divisor >> 1;
        }
        numerator
    }

    // fn rem(self, other: Self) -> Self::Output {
    //     let mut numerator = self;
    //     while numerator >= other {
    //         // bigu288::new() is equal to 0
    //         numerator = numerator - other;
    //     }
    //     numerator // Remainder
    // }
}
// TODO: Do this in constant time!
impl Div for u288 {
    type Output = u288;
    fn div(self, other: Self) -> Self::Output {
        let mut numerator = self;
        let mut divisor = other;
        let mut quotient = u288::new(); // 0
        let one = u288::from_hex("1");

        // Align divisor to msb of numerator and store the shift amount in n
        let mut n: usize = 0;
        let mut flag = 1; // Flag to detect when msb has been hit
        for i in (0..numerator.0.len()).rev() {
            // Iterate over the bytes backwards
            n += flag & (numerator.0[i] != 0 && divisor.0[i] == 0) as usize;
            flag &= !(divisor.0[i] != 0) as usize;
        }
        divisor = divisor << n; // TODO: Make this constant time!

        // TODO: This is temporary! Need to find a more permament solution
        let mut n: i64 = n as i64;

        // Keep shifting divisor to the right (decrease, in-memory left shift due to le)
        while other <= numerator {
            // Subtract until not possible anymore, then add to quotient
            let mut i = u288::new();
            while divisor <= numerator {
                numerator = numerator - divisor;
                i = i + one;
            }
            quotient = quotient + i << n as usize;
            n -= 1;
            divisor = divisor >> 1;
        }
        quotient
    }

    // fn div(self, other: Self) -> Self::Output {
    //     let mut quotient = u288::new();
    //     let mut numerator = self;
    //     while numerator >= other {
    //         // bigu288::new() is equal to 0
    //         numerator = numerator - other;
    //         quotient = quotient + u288::from_hex("1");
    //     }
    //     quotient
    // }
}

// I don't actually know if a simple == is constant time, but to be on the safe side I implemented
// a constant time loop.
impl PartialEq<u288> for u288 {
    fn eq(&self, other: &u288) -> bool {
        let mut equal = 1;
        for (i, byte_self) in self.0.iter().enumerate() {
            equal &= (*byte_self == other.0[i]) as u8;
        }
        equal == 1
    }
}

// impl PartialEq<u8> for u288 {
//     fn eq(&self, other: &u8) -> bool {
//         self.0[0] == *other
//     }
// }

impl PartialOrd<u288> for u288 {
    fn lt(&self, other: &Self) -> bool {
        let mut lt = 0;
        for (i, byte_self) in self.0.iter().enumerate() {
            lt = (*byte_self < other.0[i]) as u8 | (lt & (*byte_self == other.0[i]) as u8) as u8;
        }
        lt == 1
    }
    fn gt(&self, other: &Self) -> bool {
        let mut gt = 0;
        for (i, byte_self) in self.0.iter().enumerate() {
            gt = (*byte_self > other.0[i]) as u8 | (gt & (*byte_self == other.0[i]) as u8) as u8;
        }
        gt == 1
    }
    fn le(&self, other: &Self) -> bool {
        let mut le = 1;
        for (i, byte_self) in self.0.iter().enumerate() {
            le = (*byte_self < other.0[i]) as u8 | (le & (*byte_self == other.0[i]) as u8) as u8;
        }
        le == 1
    }
    fn ge(&self, other: &Self) -> bool {
        let mut ge = 1;
        for (i, byte_self) in self.0.iter().enumerate() {
            ge = (*byte_self > other.0[i]) as u8 | (ge & (*byte_self == other.0[i]) as u8) as u8;
        }
        ge == 1
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!("implement partialcmp");
    }
}

impl Eq for u288 {}

impl u288 {
    pub fn from_slice(bytes: &[u8]) -> u288 {
        let mut big_u288 = u288::new();
        big_u288.0 = pad_array_bigu288(bytes).as_slice().try_into().unwrap();
        big_u288
    }
    pub fn from_hex(input: &str) -> u288 {
        let mut big_u288 = u288::new();
        // Iterate over the string backwards (we want little endian)
        let input_padded_le: [u8; 72] = pad_array_hex(&input.bytes().rev().collect::<Vec<_>>()[..]);
        for (i, char) in input_padded_le.iter().enumerate() {
            let hex_digit = u8::from_str_radix(
                &String::from_utf8(vec![*char]).unwrap_or("0".to_string()),
                16,
            )
            .unwrap_or(0);
            big_u288.0[i / 2] += hex_digit << 4 * (i % 2);
        }
        big_u288
    }
    pub fn to_hex(&self) -> String {
        let mut out = String::new();
        for byte in self.get_bytes().iter().rev() {
            out += &format!("{:x}{:x}", byte >> 4, byte & 15);
        }
        out
    }
    pub fn get_bytes(&self) -> [u8; 36] {
        self.0
    }
    pub fn new() -> u288 {
        u288([0; 36])
    }
}

fn pad_array_hex(input: &[u8]) -> [u8; 72] {
    let mut padded = [0u8; 72]; // TODO: Make this configurable
    padded[..input.len()].copy_from_slice(input);
    padded
}

fn pad_array_bigu288(input: &[u8]) -> [u8; 36] {
    let mut padded = [0u8; 36]; // TODO: Make this configurable
    padded[..input.len()].copy_from_slice(input);
    padded
}

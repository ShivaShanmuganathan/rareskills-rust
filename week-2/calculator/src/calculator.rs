use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub};

// Generic Calculator struct
#[derive(Debug)]
pub struct Calculator<T> {
    pub x: T,
    pub y: T,
}

// Generic traits
pub trait AdditiveOperations<T> {
    fn add(&self) -> T;
    fn sub(&self) -> T;
}

pub trait MultiplicativeOperations<T> {
    fn mul(&self) -> T;
    fn div(&self) -> Option<T>;
}

pub trait BinaryOperations<T> {
    fn and(&self) -> T;
    fn or(&self) -> T;
    fn xor(&self) -> T;
}

// Implement traits for Calculator<T>
impl<T> AdditiveOperations<T> for Calculator<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    fn add(&self) -> T {
        self.x + self.y
    }

    fn sub(&self) -> T {
        self.x - self.y
    }
}

impl<T> MultiplicativeOperations<T> for Calculator<T>
where
    T: Copy + Mul<Output = T> + Div<Output = T> + PartialEq + From<u8>,
{
    fn mul(&self) -> T {
        self.x * self.y
    }

    fn div(&self) -> Option<T> {
        if self.y == T::from(0u8) {
            None
        } else {
            Some(self.x / self.y)
        }
    }
}

// Implement Binary Operations only for integer-like types
impl<T> BinaryOperations<T> for Calculator<T>
where
    T: Copy + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
{
    fn and(&self) -> T {
        self.x & self.y
    }

    fn or(&self) -> T {
        self.x | self.y
    }

    fn xor(&self) -> T {
        self.x ^ self.y
    }
}

// Display for Calculator
impl<T> fmt::Display for Calculator<T>
where
    T: fmt::Display
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let div_result = self
            .div()
            .map_or("undefined".to_string(), |v| v.to_string());
        write!(
            f,
            "Addition: {}\nSubtraction: {}\nMultiplication: {}\nDivision: {}\nAND: {}\nOR: {}\nXOR: {}",
            self.add(),
            self.sub(),
            self.mul(),
            div_result,
            self.and(),
            self.or(),
            self.xor()
        )
    }
}

// New function to print all operations
pub fn print_output<T>(input: &Calculator<T>)
where
    T: fmt::Display
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>,
{
    println!("{}", input);
}

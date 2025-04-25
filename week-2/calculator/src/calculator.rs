use std::fmt;

// Debug can be auto-generated
#[derive(Debug)]
pub struct Calculator {
    pub x: i32,
    pub y: i32,
}

trait AdditiveOperations {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
}

trait MultiplicativeOperations {
    fn mul(&self) -> i32;
    fn div(&self) -> Option<i32>;
}

trait BinaryOperations {
    fn and(&self) -> i32;
    fn or(&self) -> i32;
    fn xor(&self) -> i32;
}

impl AdditiveOperations for Calculator {
    fn add(&self) -> i32 {
        self.x + self.y
    }

    fn sub(&self) -> i32 {
        self.x - self.y
    }
}

impl MultiplicativeOperations for Calculator {
    fn mul(&self) -> i32 {
        self.x * self.y
    }

    fn div(&self) -> Option<i32> {
        if self.y == 0 {
            None
        } else {
            Some(self.x / self.y)
        }
    }
}

impl BinaryOperations for Calculator {
    fn and(&self) -> i32 {
        self.x & self.y
    }

    fn or(&self) -> i32 {
        self.x | self.y
    }

    fn xor(&self) -> i32 {
        self.x ^ self.y
    }
}

impl fmt::Display for Calculator {
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

// fn main() {
//     let calculator = Calculator { x: 1, y: 2 };
//     println!("calculator:\n{}", calculator);
// }

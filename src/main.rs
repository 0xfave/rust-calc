use std::fmt;

use serde::Serialize;

#[derive(Serialize, Debug)]
struct Calculator {
    x: i32,
    y: i32,
}

trait AdditiveOperations {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
}

trait MultiplicativeOperations {
    fn div(&self) -> i32;
    fn mul(&self) -> i32;
}

trait BinaryOperations {
    fn bitwise_and(&self) -> i32;
    fn bitwise_or(&self) -> i32;
    fn bitwise_xor(&self) -> i32;
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
    fn div(&self) -> i32 {
        self.x / self.y
    }

    fn mul(&self) -> i32 {
        self.x * self.y
    }
}

impl BinaryOperations for Calculator {
    fn bitwise_and(&self) -> i32 {
        self.x & self.y
    }

    fn bitwise_or(&self) -> i32 {
        self.x | self.y
    }

    fn bitwise_xor(&self) -> i32 {
        self.x ^ self.y
    }
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Addition: {}\nSubstraction: {}\nMultilication: {}\nDivision: {}\nAND: {}\nOR: {}\nXOR: {}",
            self.add(),
            self.sub(),
            self.mul(),
            self.div(),
            self.bitwise_and(),
            self.bitwise_or(),
            self.bitwise_xor()
        )
    }
}

fn print_output<T>(calculator: &T)
where
    T: AdditiveOperations + MultiplicativeOperations + BinaryOperations,
{
    println!("Addition: {}", calculator.add());
    println!("Subtraction: {}", calculator.sub());
    println!("Multiplication: {}", calculator.mul());
    println!("Division: {}", calculator.div());
    println!("AND: {}", calculator.bitwise_and());
    println!("OR: {}", calculator.bitwise_or());
    println!("XOR: {}", calculator.bitwise_xor());
}

fn main() {
    let calculator = Calculator { x: 60, y: 5 };

    println!("Calculator: {}", calculator);
    println!("\nUsing print_output function:");
    print_output(&calculator);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.add(), 15);
    }

    #[test]
    fn test_subtraction() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.sub(), 5);
    }

    #[test]
    fn test_multiplication() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.mul(), 50);
    }

    #[test]
    fn test_division() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.div(), 2);
    }

    #[test]
    fn test_bitwise_and() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.bitwise_and(), 0); // 10 & 5 = 0
    }

    #[test]
    fn test_bitwise_or() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.bitwise_or(), 15); // 10 | 5 = 15
    }

    #[test]
    fn test_bitwise_xor() {
        let calculator = Calculator { x: 10, y: 5 };
        assert_eq!(calculator.bitwise_xor(), 15); // 10 ^ 5 = 15
    }
}

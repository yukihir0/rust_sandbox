use std::fmt;

enum Result {
    Number(u64),
    Fizz,
    Buzz,
    FizzBuzz,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Result::Number(n)  => write!(f, "{}", n),
            Result::Fizz       => write!(f, "Fizz"),
            Result::Buzz       => write!(f, "Buzz"),
            Result::FizzBuzz   => write!(f, "Fizz Buzz"),
        }
    }
}

impl fmt::Debug for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Result::Number(n)  => write!(f, "{}", n),
            Result::Fizz       => write!(f, "Fizz"),
            Result::Buzz       => write!(f, "Buzz"),
            Result::FizzBuzz   => write!(f, "Fizz Buzz"),
        }
    }
}

struct FizzBuzzGenerator {
    num: u64,
}

impl FizzBuzzGenerator {
    fn new() -> FizzBuzzGenerator {
        FizzBuzzGenerator { num: 0 }
    }
}

impl Iterator for FizzBuzzGenerator {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.num = self.num + 1;
        
        if self.num % 15 == 0 {
            Some(Result::FizzBuzz)
        } else if self.num % 3 == 0 {
            Some(Result::Fizz)
        } else if self.num % 5 == 0 {
            Some(Result::Buzz)
        } else {
            Some(Result::Number(self.num))
        }
    }
}

fn main() {
    let fizzbuzz= FizzBuzzGenerator::new();
    for n in fizzbuzz.take(20) {
        println!("{}", n)
    }
}

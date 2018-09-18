use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

// 引数が1つのコンストラクタがある場合は、
// 対応する型からのFromトレイトとして実装するとよい
// A::new(B) => From<B> for A => A::from(B)
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Fromトレイトを実装するとIntoトレイトは自動的に実装される
/*
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self } // Number::from(self)と等価
    }
}
*/

fn main() {
    // From trait
    let num1 = Number::from(10);
    println!("My number is {:?}", num1);

    // Into trait
    let num2: Number = 20.into(); // データ型を宣言しないとコンパイルエラーになる
    println!("My number is {:?}", num2);
}

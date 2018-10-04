extern crate rand;

use rand::distributions::Alphanumeric;
use rand::prelude::*;

fn random_string(n: usize) -> String {
    use std::iter;

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(n)
        .collect()
}

fn main() {
    let n = 10;

    println!("{}", random_string(n));
}

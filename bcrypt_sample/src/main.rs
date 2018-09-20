extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let password = "password";
    let digest = hash(password, DEFAULT_COST).unwrap();
    println!("{} => {}", password, digest);
    
    let is_valid = verify(password, &digest).unwrap();
    println!("{}", is_valid); 

    let invalid_password = "invalid";
    let is_valid = verify(invalid_password, &digest).unwrap();
    println!("{}", is_valid); 
}

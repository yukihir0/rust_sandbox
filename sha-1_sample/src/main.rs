extern crate sha1;

use sha1::{Digest, Sha1};

fn main() {
    println!("{:x}", Sha1::digest_str("password"));
}

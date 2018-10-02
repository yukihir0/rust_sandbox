extern crate uuid;

use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
}

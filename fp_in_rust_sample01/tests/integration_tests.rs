extern crate fp_in_rust_sample01;
extern crate timebomb;

use timebomb::timeout_ms;

#[test]
fn test_main() {
    timeout_ms(|| {
        fp_in_rust_sample01::run_simulation();
    }, 300000);
}

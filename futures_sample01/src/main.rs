extern crate futures;
extern crate future_by_example;
extern crate tokio_core;

fn main() {
    // Reference: https://paulkernfeld.com/2018/01/20/future-by-example.html
    sample01();
    sample02();
    sample03();
    sample04();
    sample05();
    sample06();
    sample07();
    sample08();
    sample09();
    sample10();
    
    // Reference: Network programming in Rust.
    //sample11();

    sample12();
    sample13();
    sample14();
}

fn sample01() {
    use futures::Future;
    use future_by_example::new_example_future;

    let future = new_example_future();

    let expected = Ok(2);
    assert_eq!(future.wait(), expected);
}

fn sample02() {
    use futures::Future;
    use future_by_example::new_example_future;

    let future = new_example_future();
    let mapped = future.map(|i| i * 3);

    let expected = Ok(6);
    assert_eq!(mapped.wait(), expected);
}

fn sample03() {
    use futures::Future;
    use future_by_example::*;

    let good = new_example_future();
    let bad = new_example_future_err();
    let both = good.and_then(|_good| bad);

    let expected = Err(ExampleFutureError::Oops);
    assert_eq!(both.wait(), expected);
}

fn sample04() {
    use futures::Future;
    use future_by_example::new_example_future;

    let future1 = new_example_future();
    let future2 = new_example_future();

    let joined = future1.join(future2);
    let (value1, value2) = joined.wait().unwrap();
    assert_eq!(value1, value2);
}

fn sample05() {
    use futures::Future;
    use futures::future::ok;

    let future = ok::<_, ()>(String::from("hello"));
    assert_eq!(Ok(String::from("hello")), future.wait());
}

fn sample06() {
    use futures::future::ok;
    use futures::Future;

    let expected: Result<u64, ()> = Ok(6);
    assert_eq!(
        ok(5).join(ok(7)).map(|(x, y)| x + y).map(|z| z / 2).wait(),
        expected
    )
}

fn sample07() {
    use futures::future::ok;
    use futures::Future;
    use futures::Map;

    let expected: Result<_, ()> = Ok(6);
    let twelve: Map<_, _> = ok(5).join(ok(7)).map(|(x, y)| x + y);
    assert_eq!(twelve.map(|z| z / 2).wait(), expected)
}

fn sample08() {
    use futures::Future;
    use futures::future::ok;

    fn make_twelve() -> Box<Future<Item=u64, Error=()>> {

        Box::new(ok(5).join(ok(7)).map(|(x, y)| x + y))
    }

    let twelve = make_twelve();
    assert_eq!(twelve.map(|z| z / 2).wait(), Ok(6))
}

fn sample09() {
    use futures::Future;
    use futures::future::ok;

    let make_twelve = || {

        ok(5).join(ok(7)).map(|(x, y)| x + y)
    };

    let expected: Result<u64, ()> = Ok(6);
    let twelve = make_twelve();
    assert_eq!(twelve.map(|z| z / 2).wait(), expected)
}

fn sample10() {
    use tokio_core::reactor::Core;
    use futures::future::lazy;

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let future = lazy(|| {
        handle.spawn(lazy(|| {
            Ok(()) // Ok(()) implements FromResult
        }));
        Ok(2)
    });
    let expected: Result<_, ()> = Ok(2usize);
    assert_eq!(core.run(future), expected);
}

fn sample11() {
    use std::io;
    use futures::Future;
    
    fn check_prime_boxed(n: u64) -> Box<Future<Item = bool, Error = io::Error>> {
        for i in 2..n {
            if n % i == 0 { return Box::new(futures::future::ok(false)); }
        }
        Box::new(futures::future::ok(true))
    }

    fn check_prime_impl_trait(n: u64) -> impl Future<Item=bool, Error=io::Error> {
        for i in 2..n {
            if n % i == 0 { return futures::future::ok(false); }
        }
        futures::future::ok(true)
    }

    let input: u64 = 58466453;
    println!("Right before first call");
    let res_one = check_prime_boxed(input);
    println!("Called check_prime_boxed");
    let res_two = check_prime_impl_trait(input);
    println!("Called check_prime_impl_trait");
    println!("Results are {} and {}", res_one.wait().unwrap(),
    res_two.wait().unwrap());
}

fn sample12() {
    use futures::Future;
    use future_by_example::*;

    let good = new_example_future();
    let bad = new_example_future_err();
    let both = good
        .and_then(|_good| bad)
        .or_else(|_| Ok(1))
        .and_then(|x| Ok(x+1))
        .and_then(|x| Ok(x+1))
        .and_then(|_| new_example_future_err());

    let expected = Err(ExampleFutureError::Oops);
    assert_eq!(both.wait(), expected);
}

fn sample13() {
    use futures::Future;
    use futures::future::ok;

    #[derive(Debug, PartialEq)]
    struct Hoge { id: u32 }

    let hoge = ok::<Hoge, _>(Hoge { id: 1 });

    let mapped = hoge
        .map(|mut h| {
            // Result<Hoge, ()> => Result<Hoge, ()>
            h.id = 10;
            h
        })
        .map(|_| {
            // Result<Hoge, ()> => Result<Hoge, ()>
            Hoge { id: 100 } 
        })
        .map(|_| {
            // Result<Hoge, ()> => Result<Integer, ()>
            100
        })
        .map(|_| {
            // Result<Integer, ()> => Result<String, ()>
            "hoge".to_string() 
        })
        .map_err(|_: ()| {
            // Result<String, ()> => Result<String, String>
            Err::<String, String>("hoge_error".to_string())
        })
        .map(|_| {
            // Result<String, String> => Result<Hoge, String>
            Hoge { id: 1000 }
        });

    let expected = Ok(Hoge { id: 1000 });
    assert_eq!(mapped.wait(), expected);
}

fn sample14() {
    use futures::Future;
    use futures::future::ok;

    #[derive(Debug, PartialEq)]
    struct Hoge { id: u32 }

    let hoge = ok::<Hoge, _>(Hoge { id: 1 });

    let mapped = hoge
        .and_then(|mut h| {
            // Result<Hoge, ()> => Result<Hoge, ()>
            h.id = 10;
            Ok(h)
        })
        .and_then(|_| {
            // Result<Hoge, ()> => Result<Hoge, ()>
            Ok(Hoge { id: 100 })
        })
        .and_then(|_| {
            // Result<Hoge, ()> => Result<Integer, ()>
            Ok(100)
        })
        .and_then(|_| {
            // Result<Integer, ()> => Result<String, ()>
            Ok("hoge".to_string())
        })
        .or_else(|_: ()| {
            // Never called.
            Err("hoge_error1".to_string())
        })
        .and_then(|_| {
            // Result<Hoge, String> => Result<Hoge, String>
            Ok(Hoge { id: 1000 })
        });

    let expected = Ok(Hoge { id: 1000 });
    assert_eq!(mapped.wait(), expected);
}

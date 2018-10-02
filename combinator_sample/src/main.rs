fn main() {
    // Reference: https://hermanradtke.com/2016/09/12/rust-using-and_then-and-map-combinators-on-result-type.html
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
}

fn sample01() {
    let res: Result<usize, &'static str> = Ok(5);
    let value = res.and_then(|n: usize| Ok(n * 2));
    
    assert_eq!(Ok(10), value);
}

fn sample02() {
    let res: Result<usize, &'static str> = Err("error");
    let value = res.and_then(|n: usize| Ok(n * 2));
    
    assert_eq!(Err("error"), value);
}

fn sample03() {
    let res: Result<usize, &'static str> = Ok(0);

    let value = res
        .and_then(|n: usize| {
            if n == 0 {
                Err("cannot divide by zero")
            } else {
                Ok(n)
            }
        })
        .and_then(|n: usize| Ok(2 / n));

    assert_eq!(Err("cannot divide by zero"), value);
}

fn sample04() {
    let res: Result<Result<usize, &'static str>, &'static str> = Ok(Ok(5));

    let value = res
        .and_then(|n: Result<usize, &'static str>| {
            n
        })
        .and_then(|n: usize| {
            Ok(n * 2)
        });

    assert_eq!(Ok(10), value);
}

fn sample05() {
    let res: Vec<usize> = vec![5];
    let value: Vec<usize> = res.iter().map(|n| n * 2).collect();
    assert_eq!(vec![10], value);
}

fn sample06() {
    let res: Result<usize, &'static str> = Ok(5);
    let value: Result<usize, &'static str> = res.map(|n| n * 2);
    assert_eq!(Ok(10), value);
}

fn sample07() {
    let given: Result<i32, &'static str> = Ok(5i32);
    let desired: Result<usize, &'static str> = given.map(|n: i32| n as usize);

    assert_eq!(Ok(5usize), desired);

    let value = desired.and_then(|n: usize| Ok(n * 2));

    assert_eq!(Ok(10), value);
}

fn sample08() {
    let given: Result<i32, &'static str> = Err("an error");
    let desired: Result<usize, &'static str> = given.map(|n: i32| n as usize);

    assert_eq!(Err("an error"), desired);

    let value = desired.and_then(|n: usize| Ok(n * 2));

    assert_eq!(Err("an error"), value);
}

fn sample09() {
    enum MyError { Bad };

    let given: Result<i32, MyError> = Err(MyError::Bad);

    let desired: Result<usize, &'static str> = given
        .map(|n: i32| {
           n as usize
        })
        .map_err(|_e: MyError| {
           "bad MyError"
        });

    let value = desired.and_then(|n: usize| Ok(n * 2));

    assert_eq!(Err("bad MyError"), value);
}

fn sample10() {
    enum FooError {
        Bad,
    }

    enum BarError {
        Horrible,
    }

    let res: Result<Result<i32, FooError>, BarError> = Ok(Err(FooError::Bad));

    let value = res
        .map(|res: Result<i32, FooError>| {
            res
                .map(|n: i32| n as usize)
                .map_err(|_e: FooError| "bad FooError")
        })
        .map_err(|_e: BarError| {
            "horrible BarError"
        })
        .and_then(|n: Result<usize, &'static str>| {
            n
        })
        .and_then(|n: usize| {
            Ok(n * 2)
        });

        assert_eq!(Err("bad FooError"), value);
}

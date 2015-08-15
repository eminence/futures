
#[cfg(feature="threadpool")]
extern crate threadpool;
extern crate futures;

use futures::Future;
#[cfg(feature="threadpool")]
use futures::FuturePool;

#[cfg(feature="threadpool")]
#[test]
fn test_pool1() {
    use threadpool::ThreadPool;

    let tpool = ThreadPool::new(4);
    let fpool = FuturePool::from_pool(tpool);

    let future: Future<i32> = fpool.spawn(|| {
        let a=2; let b=3;
        return a*b
    });

    assert_eq!(future.unwrap(), 6 as i32);
}

#[cfg(feature="threadpool")]
#[test]
fn test_pool2() {
    use threadpool::ThreadPool;
    use std::env::var_os;;

    let tpool = ThreadPool::new(4);
    let fpool = FuturePool::from_pool(tpool);

    let future: Future<String> = fpool.spawn(|| {
        if let Some(s) = var_os("USER") {
            s.into_string().unwrap()
        } else {
            "unknown".to_owned()
        }
    });

    assert_eq!(future.unwrap(), "achin");
}

#[cfg(feature="threadpool")]
#[test]
fn test_pool3() {
    use threadpool::ThreadPool;
    use std::thread::sleep_ms;

    let tpool = ThreadPool::new(4);
    let fpool = FuturePool::from_pool(tpool);

    let mut future: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 1 });
    let future1: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 2 });
    let future2: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 3 });
    let future3: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 4 });
    let future4: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 5 });
    let future5: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 6 });
    let future6: Future<i32> = fpool.spawn(|| { sleep_ms(2500); 7 });

    assert_eq!(future.done(), false);
    assert_eq!(future.unwrap(), 1 as i32);
    assert_eq!(future1.unwrap(), 2 as i32);
    assert_eq!(future2.unwrap(), 3 as i32);
    assert_eq!(future3.unwrap(), 4 as i32);
    assert_eq!(future4.unwrap(), 5 as i32);
    assert_eq!(future5.unwrap(), 6 as i32);
    assert_eq!(future6.unwrap(), 7 as i32);
}


#[test]
fn test_4() {
    use std::thread::sleep_ms;

    let mut future: Future<i32> = Future::spawn(|| { sleep_ms(2500); 1});
    assert_eq!(future.done(), false);
    assert_eq!(future.unwrap(), 1 as i32);
}

#[test]
fn test_5() {
    use std::thread::sleep_ms;

    let mut future: Future<i32> = Future::spawn(|| { sleep_ms(2500); 1});
    assert_eq!(future.done(), false);
    sleep_ms(3000);
    assert_eq!(future.done(), true);
    assert_eq!(future.unwrap(), 1 as i32);
}

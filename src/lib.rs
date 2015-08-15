//! A simple implementation of a Futures object, which represents the result
//! of a computation that may not be ready yet
//!
//! # Examples
//! ```
//! use futures::Future;
//! use std::thread::sleep_ms;
//!
//! let futureA: Future<i32> = Future::spawn(|| {sleep_ms(2500); 2});
//! let futureB: Future<i32> = Future::spawn(|| {sleep_ms(2500); 4});
//! assert_eq!(futureA.unwrap() + futureB.unwrap(), 6);
//! ```
//!


#[cfg(feature="threadpool")]
extern crate threadpool;

#[cfg(feature="threadpool")]
use threadpool::ThreadPool;
use std::thread;

use std::sync::mpsc::{channel, Receiver};
use std::fmt;

pub struct Future<T> {
    /// Holds the result, if available
    result: Option<T>,

    /// The result will be received down this channel
    receiver: Receiver<T>,

    /// If the result is already saved in `result`
    done: bool
}

impl<T> fmt::Debug for Future<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Future({:?})", self.result)
    }
}


#[cfg(feature="threadpool")]
/// A wrapper around a thread pool.  
///
/// If you do not want this feature, disable the `threadpool` feature
///
pub struct FuturePool {
    pool: ThreadPool
}

#[cfg(feature="threadpool")]
impl FuturePool {
    pub fn from_pool(pool: ThreadPool) -> FuturePool {
        FuturePool{pool: pool}
    }

    /// Creates a new Future to be executed in a thread pool
    pub fn spawn<F, T>(&self, func: F) -> Future<T> 
        where F: FnOnce() -> T,
              F: Send + 'static,
              T: Send + 'static {
            let (tx, rx) = channel();
            self.pool.execute(move || {
                let result:T = func();
                tx.send(result).unwrap();

            });
            Future::make(rx)
        }
    
}

impl<T> Future<T> where T: Send {
    fn make(receiver: Receiver<T>) -> Future<T> {
        Future { result: None, receiver: receiver, done: false}
    }

    /// Creates a few future
    pub fn spawn<F>(func :F) -> Future<T>
        where F: FnOnce() -> T,
              F: Send + 'static,
              T: Send + 'static {

                  let (tx, rx) = channel();
                  thread::spawn(move || {
                      let result:T = func();
                      tx.send(result).unwrap();
                  });
                  Future::make(rx)
              }
    
    fn try_get(&mut self) {
        if !self.done {
            self.result = match self.receiver.try_recv() {
                Ok(t) => {self.done = true; Some(t) },
                Err(_) => None
            };
        }
    }

    
    /// Waits until the result is ready
    pub fn unwrap(self) -> T {
        if self.done { 
            self.result.unwrap()
        } else {
            self.receiver.recv().unwrap() 
        }
    }


    /// Checks to see if the result is ready, then returns if the result is ready.
    ///
    /// Does not block
    pub fn done(&mut self) -> bool {
        self.try_get();
        self.done
    }
        
    
}


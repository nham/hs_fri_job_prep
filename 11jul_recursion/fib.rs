extern crate test;

use std::collections::hashmap::HashMap;
use test::Bencher;

fn naive_fib(n: uint) -> uint {
    if n < 2 {
        n
    } else {
        naive_fib(n-1) + naive_fib(n-2) 
    }
}


struct FibMemoizer {
    mem: HashMap<uint, uint>,
}

impl FibMemoizer {
    fn new() -> FibMemoizer {
        FibMemoizer { mem: HashMap::new() }
    }

    fn memoizing_fib(&mut self, n: uint) -> uint {
        if !self.mem.contains_key(&n) {
            let res = self.fib(n);
            self.mem.insert(n, res);
            res
        } else {
            *self.mem.find(&n).unwrap()
        }
    }

    fn fib(&mut self, n: uint) -> uint {
        if n < 2 {
            n
        } else {
            self.memoizing_fib(n-1) + self.memoizing_fib(n-2) 
        }
    }
}

struct FibIter {
    prev: uint,  // n - 1
    prev2: uint, // n - 2
}

impl FibIter {
    fn new() -> FibIter {
        FibIter { prev: 1u, prev2: 0u }
    }
}

impl Iterator<uint> for FibIter {
    fn next(&mut self) -> Option<uint> {
        let tmp = self.prev2;
        self.prev2 = self.prev;
        self.prev = tmp + self.prev2;
        Some(tmp)
    }
}

fn fib_iter(n: uint) -> uint {
    let mut it = FibIter::new();
    let mut x = 0u;
    for i in range(0, n+1) { x = it.next().unwrap(); }
    x
}


#[bench]
fn bench_naive(b: &mut Bencher) {
    b.iter(|| naive_fib(25) );
}

#[bench]
fn bench_memoizing(b: &mut Bencher) {
    let mut fm = FibMemoizer::new();
    b.iter(|| fm.fib(25) );
}

#[bench]
fn bench_iter(b: &mut Bencher) {
    let mut fm = FibMemoizer::new();
    b.iter(|| fib_iter(25) );
}

fn main() {
    let mut fm = FibMemoizer::new();
    for i in range(0u, 20) {
        println!("Naive: {}, Memoizing: {}, Iter: {}", 
                 naive_fib(i), fm.fib(i), fib_iter(i));
    }
}

// That's right.
#![allow(irrefutable_let_patterns)]
use std::{env, fs};
use ramp::int;
use rayon::prelude::*;
fn main() {
    let primes: Vec<int> = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
                            31, 37, 41, 43, 47, 53, 59, 61, 67,
                            71, 73, 79, 83, 89, 97].into_iter()
                                                   .map(|x| int::from(*x))
                                                   .collect();
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let n: int = contents.lines().nth(0).unwrap().parse().unwrap();
    if n < 10000 {
        let (p, q) = look_up(&n, &primes);
        println!("{}={}*{}", n, p, q);
    }
    else {
        let (p, q) = pollard_brent(&n);
        println!("{}={}*{}", n, p, q);
    }
}

/* https://stackoverflow.com/a/2274520/9221785 */
fn look_up(n: &int, ps: &Vec<int>) -> (int, int) {
    for p in ps.iter() {
		if let (q, r) = n.divmod(p) {
            if r == 0 { return (p.clone(), q) };
        }
    }
    (int::one(), n.clone())
}

/* Refs:
 * +http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf
 * +https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/
 */
fn pollard_brent(num: &int) -> (int, int) {
    let (mut x, mut y) = (int::from(2), int::from(2));
    let (mut d, mut q) = (int::one(), int::one());
    let (mut ys, mut r) = (int::zero(), 1);
    const M: i32 = 71;
    let f = |i: &int| (i.pow_mod(&int::from(2), &num) + M) % num;
    while d == 1 {
        x = f(&y);
        for _ in 0..r {
            y = f(&y);
        }
        let mut k = 0_i32;
        while k < r && d == 1 {
            for _ in 0..(M.min(r - k)) {
                y = f(&y);
                q = q * (&x - &y).abs() % num;
            }
            ys = y.clone();
            d = q.gcd(num);
            k += M;
        }
        r *= 2;
    }
    if d == *num {
        loop {
            ys = f(&ys);
            d = ((&x - &ys).abs()).gcd(num);
            if d > 1 {
                break;
            }
        }
    }
    (d.clone(), num/d)
}

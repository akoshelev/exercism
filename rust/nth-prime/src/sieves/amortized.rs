extern crate bit_vec;

use self::bit_vec::BitVec;
use std::cmp;

pub fn nth(n: u32) -> Option<u32> {
    match n {
        0 => None,
        n => Primes::new().into_iter().nth((n - 1) as usize)
    }
}

struct Primes {
    sieve: BitVec,
    pos: usize,
}

impl Primes {
    fn new() -> Primes {
        let mut sieve = BitVec::from_elem(2usize.pow(10), true);

        sieve.set(0, false);
        sieve.set(1, false);

        mark_primes(&mut sieve);

        Primes { sieve, pos: 1 }
    }

    fn grow(&mut self) -> Option<usize> {
        let old_size = self.sieve.len();
        let new_size = cmp::min(u32::max_value() as usize, 2 * old_size);

        if new_size == old_size { None }
        else {
            self.sieve.grow(new_size - old_size, true);

            mark_primes(&mut self.sieve);

            Some(new_size)
        }
    }

    fn maybe_grow(&mut self) -> Option<usize> {
        if self.sieve.len() - 1 == self.pos {
            self.grow()
        } else {
            None
        }
    }

    fn is_on_prime(&self) -> bool {
        self.sieve[self.pos]
    }

}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            while self.pos < self.sieve.len() - 1 {
                self.pos += 1;
                if self.is_on_prime() {
                    return Some(self.pos as u32)
                }
            }

            match self.grow() {
                Some(_) => continue,
                None => return None
            }
        }
    }
}

fn mark_primes(vec: &mut BitVec) -> () {

    let root = (vec.len() as f32).sqrt() + 1f32;

    for i in 2..root as usize {
        if vec[i] {
            for j in (i*i..vec.len()).step_by(i) {
                vec.set(j, false);
            }
        }
    }
}


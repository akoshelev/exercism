
mod sieves;

pub fn nth(n: u32) -> Option<u32> {
//    sieves::appr::nth(n)
    sieves::amortized::nth(n)
//    nth_sieve_iter(n)
}


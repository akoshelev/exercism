/// Calculates nth prime number using Sieve of Eratosthenes + approximation for n-th number
/// (prime theorem)
pub fn nth(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => Some(2),
        2 => Some(3),
        y => sieve(y)
    }
}

fn sieve(n: u32) -> Option<u32> {
    // get approximation for nth number
    let upper_bound = nth_appr(n);
    let mut sieve_vec = vec![true; upper_bound];
    sieve_vec[0] = false;
    sieve_vec[1] = false;

    // step 1 - fill up sieve (a bit of oxymoron)
    for i in 2..(upper_bound as f32).sqrt() as usize + 1 {
        if sieve_vec[i] {
            for j in (i * i..upper_bound).step_by(i) {
                sieve_vec[j] = false
            }
        }
    };

    // step 2 - find nth prime number
    sieve_vec.iter()
        .enumerate()
        .filter_map(|(i, &prime)| Some(i as u32).filter(|_| prime))
        .nth((n as usize) - 1)
}

/// Returns an approximation (upper bound) for nth prime number
fn nth_appr(n: u32) -> usize {
    let n = n as f32;

    /*
     * Prime number theorem
     * https://en.wikipedia.org/wiki/Prime_number_theorem#Approximations_for_the_nth_prime_number
     *
     * Calculate approximation for the nth prime number
     * (we don't need to go over sqrt(upper_bound) in outer loop)
     */
    (n * (n.ln() + n.ln().ln()) + 3f32) as usize // as theorem assumes n >= 6
}


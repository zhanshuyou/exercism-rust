pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut candidate = 3;
    while primes.len() <= n as usize {
        if primes.iter().all(|prime| candidate % prime != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes[n as usize]
}

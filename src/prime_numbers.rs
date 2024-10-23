pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn sieve_of_eratosthenes(limit: i64) -> Vec<i64> {
    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    for num in 2..=((limit as f64).sqrt() as i64) {
        if sieve[num as usize] {
            let mut multiple = num * num;
            while multiple <= limit {
                sieve[multiple as usize] = false;
                multiple += num;
            }
        }
    }
    sieve.iter().enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(num, _)| num as i64)
        .collect()
}

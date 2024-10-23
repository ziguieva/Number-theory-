pub fn is_perfect_number(n: i64) -> bool {
    let sum_of_divisors: i64 = (1..n).filter(|&i| n % i == 0).sum();
    sum_of_divisors == n
}

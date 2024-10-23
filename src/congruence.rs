pub fn solve_congruence(a: i64, b: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = bezout_coefficients(a, m);
    if b % gcd != 0 {
        return None;
    }
    Some((x * (b / gcd)) % m)
}

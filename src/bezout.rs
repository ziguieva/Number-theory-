pub fn bezout_coefficients(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (gcd, x1, y1) = bezout_coefficients(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (gcd, x, y)
}

pub fn modular_inverse(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = bezout_coefficients(a, m);
    if gcd != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

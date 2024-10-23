pub fn mod_exp(base: i64, exponent: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent /= 2;
        base = (base * base) % modulus;
    }
    result
}

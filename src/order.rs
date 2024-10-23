pub fn order_in_group(a: i64, m: i64) -> Option<i64> {
    if gcd(a, m) != 1 {
        return None;
    }
    let mut order = 1;
    let mut value = a % m;
    while value != 1 {
        value = (value * a) % m;
        order += 1;
    }
    Some(order)
}

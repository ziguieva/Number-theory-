pub fn chinese_remainder_theorem(congruences: &[i64], moduli: &[i64]) -> i64 {
    let prod = moduli.iter().product::<i64>();
    let mut result = 0;
    for (&b_i, &m_i) in congruences.iter().zip(moduli.iter()) {
        let p = prod / m_i;
        let inv = modular_inverse(p, m_i).unwrap();
        result += b_i * inv * p;
    }
    result % prod
}

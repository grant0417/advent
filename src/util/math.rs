use num::BigUint;

pub fn lcm<N, I>(numbers: I) -> BigUint
where
    N: Into<BigUint>,
    I: Iterator<Item = N>,
{
    numbers.fold(BigUint::from(1_usize), |acc, f| {
        num::integer::lcm(acc, f.into())
    })
}

pub fn gcd<N, I>(numbers: I) -> BigUint
where
    N: Into<BigUint>,
    I: Iterator<Item = N>,
{
    numbers.fold(BigUint::from(0_usize), |acc, f| {
        num::integer::gcd(acc, f.into())
    })
}

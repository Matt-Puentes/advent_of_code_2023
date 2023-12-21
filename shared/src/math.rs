use num::Integer;

pub fn gcd<T>(i: &T, o: &T) -> T
where
    T: Integer,
{
    Integer::gcd(i, o)
}

pub fn lcm<T>(i: &T, o: &T) -> T
where
    T: Integer,
{
    Integer::lcm(i, o)
}

pub trait Bound {
    const MAX: Self;
    const MAX_AS_U128: u128;
    const MAX_AS_I128: i128;
    const MIN: Self;
    const MIN_AS_U128: u128 = 0;
    const MIN_AS_I128: i128 = 0;
}
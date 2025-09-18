use super::*;

mode!(
    // internally will have at least 1 precision, but
    // when displayed or converted, will always yield
    // a whole number
    Cardinal
);
use super::*;

pub trait Int 
where
    Self: Sized,
    Self: Num,
    Self: Checked,
    Self: Overflowing,
    Self: Eq,
    Self: PartialEq,
    Self: Bits,
    Self: Bound,
    Self: QuickAccess {}

impl Int for u8 {}

impl Checked for u8 {}

impl CheckedAdd for u8 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs)
    }
}

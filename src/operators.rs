use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

struct Addition<T>
where
    T: Sized,
{
    op1: T,
    op2: T,
}

struct Negation<T>
where
    T: Sized,
{
    op1: T,
    op2: T,
}
struct Multiplication {}
struct Division {}

impl<T: Sized, U: Sized> PartialEq<Negation<U>> for Addition<T> {
    fn eq(&self, _: &Negation<U>) -> bool {
        true
    }
}
impl<T: Sized, U: Sized> PartialOrd<Negation<U>> for Addition<T> {
    fn partial_cmp(&self, _: &Negation<U>) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_neg_comparison() {
        assert!(Addition { op1: 1, op2: 2 } == Negation { op1: 1, op2: 2 });
        assert!(!(Addition { op1: 1, op2: 2 } > Negation { op1: 1, op2: 2 }));

        // assert!(Negation {} == Addition { op1: 1, op2: 2 });
    }

    // #[test]
    // fn mul_div_comparison() {
    //     assert_ne!(Multiplication)
    // }
}

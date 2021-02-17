use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(PartialOrd, PartialEq)]
pub struct UncheckOrd<T: PartialOrd + PartialEq>(pub T);

impl<T: PartialOrd + PartialEq> Eq for UncheckOrd<T> {}

impl<T: PartialOrd + PartialEq> Ord for UncheckOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl<T: PartialOrd + PartialEq + Debug> Debug for UncheckOrd<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UncheckOrd({:?})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::UncheckOrd;

    #[test]
    fn it_works() {
        let v = [1.0, 3.0, 2.0];
        let mut v: Vec<_> = v.iter().copied().map(|x| UncheckOrd(x)).collect();

        v.sort();

        assert_eq!(
            v,
            vec![
                UncheckOrd(1.0_f64),
                UncheckOrd(2.0_f64),
                UncheckOrd(3.0_f64)
            ]
        );
    }
}

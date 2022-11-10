use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use crate::BigInt;

#[derive(Clone)]
pub struct List<T>(pub(crate) Vec<T>);

#[macro_export]
macro_rules! list {
	() => { List::new() };
	($start:expr $(,$a:expr)*) => { specr::hidden::vec_to_list(vec![$start $(,$a)* ]) };
	($a:expr ; $b:expr) => {
        specr::hidden::vec_to_list(
            vec![$a;
                specr::hidden::bigint_to_usize(BigInt::from($b))
            ]
        )
    };
}

impl<T> IntoIterator for List<T> {
    type IntoIter = <Vec::<T> as IntoIterator>::IntoIter;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A> {
        let v: Vec<A> = iter.into_iter().collect();
        List(v)
    }
}

impl<T> Index<BigInt> for List<T> {
    type Output = T;

    fn index(&self, other: BigInt) -> &T {
        let other = crate::hidden::bigint_to_usize(other);
        &self.0[other]
    }
}

impl<T> IndexMut<BigInt> for List<T> {
    fn index_mut(&mut self, other: BigInt) -> &mut T {
        let other = crate::hidden::bigint_to_usize(other);
        &mut self.0[other]
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List(Vec::new())
    }

    pub fn last(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }
}

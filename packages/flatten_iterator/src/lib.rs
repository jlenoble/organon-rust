pub trait IteratorExt: Iterator {
    fn our_flatten(self) -> Flatten<Self> where Self: Sized, Self::Item: IntoIterator;
}

impl<T: Iterator> IteratorExt for T {
    fn our_flatten(self) -> Flatten<Self> where Self: Sized, Self::Item: IntoIterator {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter> where I: IntoIterator, I::Item: IntoIterator {
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O> where O: IntoIterator, O::Item: IntoIterator {
    outer: O,
    fwd_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    bck_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O> where O: Iterator, O::Item: IntoIterator {
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            fwd_inner: None,
            bck_inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O> where O: Iterator, O::Item: IntoIterator {
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut fwd_inner_iter) = self.fwd_inner {
                if let Some(item) = fwd_inner_iter.next() {
                    return Some(item);
                }
                self.fwd_inner = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.fwd_inner = Some(next_inner.into_iter());
            } else {
                return self.bck_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator
    for Flatten<O>
    where
        O: DoubleEndedIterator,
        O::Item: IntoIterator,
        <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut bck_inner_iter) = self.bck_inner {
                if let Some(item) = bck_inner_iter.next_back() {
                    return Some(item);
                }
                self.bck_inner = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.bck_inner = Some(next_back_inner.into_iter());
            } else {
                return self.fwd_inner.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).collect::<Vec<_>>(), vec!["a"]);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn two_deep() {
        assert_eq!(flatten(vec![vec!["a", "b"]]).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn reverse_two() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_two_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_two_deep() {
        assert_eq!(
            flatten(vec![vec!["a", "b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_deep_wide() {
        assert_eq!(
            flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b2", "b1", "a3", "a2", "a1"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(
            vec![vec!["a1", "a2", "a3"], vec!["b1", "b2"], vec!["c1", "c2", "c3"]]
        );

        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("c3"));

        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("c2"));

        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("c1"));

        assert_eq!(iter.next(), Some("b1"));
        assert_eq!(iter.next_back(), Some("b2"));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 1..i));

        assert_eq!(iter.next(), Some(1));

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
    }

    #[test]
    fn flatten_flatten() {
        assert_eq!(
            flatten(
                vec![vec![vec![1], vec![2, 3, 4], vec![5, 6]], vec![vec![7]], vec![vec![8, 9]]]
            ).collect::<Vec<_>>(),
            vec![vec![1], vec![2, 3, 4], vec![5, 6], vec![7], vec![8, 9]]
        );
        assert_eq!(
            flatten(
                flatten(
                    vec![vec![vec![1], vec![2, 3, 4], vec![5, 6]], vec![vec![7]], vec![vec![8, 9]]]
                )
            ).collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn ext() {
        assert_eq!(
            vec![vec!["a"], vec!["b"]].into_iter().our_flatten().collect::<Vec<_>>(),
            vec!["a", "b"]
        );
    }
}
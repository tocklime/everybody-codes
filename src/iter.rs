use std::cmp::Ordering;

pub fn all_ix_pairs(arr_len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..arr_len - 1).flat_map(move |ix1| (ix1 + 1..arr_len).map(move |ix2| (ix1, ix2)))
}

struct Unfold<T, F> {
    current: T,
    func: F,
}

impl<T, F: Fn(&T) -> T> Iterator for Unfold<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = (self.func)(&self.current);
        std::mem::swap(&mut next, &mut self.current);
        Some(next)
    }
}
pub fn unfold<T, F: Fn(&T) -> T>(initial_value: T, step_func: F) -> impl Iterator<Item = T> {
    Unfold {
        current: initial_value,
        func: step_func,
    }
}

pub fn quick_index_by_simple_cycle<T, I>(mut iterator: I, index: usize) -> Option<T>
where
    T: Clone + PartialEq,
    I: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    let first = iterator.next()?;
    if index == 0 {
        return Some(first);
    }
    let mut last = iterator.next()?;
    let mut current_ix = 1;
    loop {
        if current_ix == index {
            return Some(last);
        }
        if last == first {
            //cycled after i iterations.
            match index % current_ix {
                0 => return Some(last),
                n => return iterator.nth(n - 1),
            }
        }
        current_ix += 1;
        last = iterator.next()?;
    }
}

pub fn all_new_greatest_with<T, TInner, F>(
    iter: impl Iterator<Item = T>,
    f: F,
) -> impl Iterator<Item = T>
where
    T: Ord,
    F: Fn(&T) -> TInner,
    TInner: PartialOrd + Copy,
{
    let mut biggest = None;
    iter.filter_map(move |i| {
        let val = f(&i);
        if Some(val) > biggest {
            biggest = Some(val);
            Some(i)
        } else {
            None
        }
    })
}

pub fn borrow_mut_twice<T>(arr: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    match a.cmp(&b) {
        Ordering::Less => {
            let (arr_a, arr_b) = arr.split_at_mut(b);
            (&mut arr_a[a], &mut arr_b[0])
        }
        Ordering::Greater => {
            let (arr_b, arr_a) = arr.split_at_mut(a);
            (&mut arr_a[0], &mut arr_b[b])
        }
        Ordering::Equal => panic!("Can't borrow twice from the same index"),
    }
}

pub fn slice_get_mut_two<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    assert_ne!(index0, index1);
    assert!(index0 < slice.len());
    assert!(index1 < slice.len());
    // SAFETY: guarantee that the indices are never the same. So it is safe to
    // have two mutable references into the Vec. We'll double check that the
    // indices are within the bounds.
    unsafe { slice_get_mut_two_unchecked(slice, index0, index1) }
}

/// # Safety
/// index0 and index1 must be < slice.len(), and must not be equal to each other.
pub unsafe fn slice_get_mut_two_unchecked<T>(
    slice: &mut [T],
    index0: usize,
    index1: usize,
) -> (&mut T, &mut T) {
    let ptr = slice.as_mut_ptr();
    let one = &mut *ptr.add(index0);
    let two = &mut *ptr.add(index1);
    (one, two)
}

pub struct PermutationBag<'a, T> {
    available: &'a [(T, usize)],
    stack: Vec<Vec<usize>>,
    k: usize,
}
impl<'a, T> PermutationBag<'a, T> {
    pub fn new(available: &'a [(T, usize)], k: usize) -> Self {
        assert!(!available.is_empty());
        assert!(available[0].1 > 0);
        let stack = (0..available.len()).map(|x| vec![x]).collect();
        Self {
            available,
            stack,
            k,
        }
    }
}
impl<'a, T> Iterator for PermutationBag<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.stack.pop() {
            if path.len() == self.k {
                return Some(path.into_iter().map(|ix| &self.available[ix].0).collect());
            }
            let mut counts: Vec<usize> = vec![0; self.available.len()];
            for &item in &path {
                counts[item] += 1;
            }
            let choices = self
                .available
                .iter()
                .enumerate()
                .filter_map(|(ix, (_, n))| {
                    (*n > counts[ix]).then_some({
                        let mut new = path.clone();
                        new.push(ix);
                        new
                    })
                });
            self.stack.extend(choices);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unfold() {
        let i: Vec<usize> = unfold(0, |x| x + 1).take(5).collect();
        assert_eq!(i, &[0, 1, 2, 3, 4]);

        let i = || unfold(0, |x| (x + 1) % 10);
        assert_eq!(quick_index_by_simple_cycle(i(), 0), Some(0));
        assert_eq!(quick_index_by_simple_cycle(i(), 1), Some(1));
        assert_eq!(quick_index_by_simple_cycle(i(), 5), Some(5));
        assert_eq!(quick_index_by_simple_cycle(i(), 10), Some(0));
        assert_eq!(quick_index_by_simple_cycle(i(), 11), Some(1));
        assert_eq!(quick_index_by_simple_cycle(i(), 19), Some(9));
        assert_eq!(quick_index_by_simple_cycle(i(), 20), Some(0));
        assert_eq!(quick_index_by_simple_cycle(i(), 21), Some(1));
        assert_eq!(quick_index_by_simple_cycle(i(), 22), Some(2));
        assert_eq!(quick_index_by_simple_cycle(i(), 23), Some(3));
        assert_eq!(quick_index_by_simple_cycle(i(), 24), Some(4));
        assert_eq!(quick_index_by_simple_cycle(i(), 25), Some(5));
        assert_eq!(quick_index_by_simple_cycle(i(), 29), Some(9));
    }
}

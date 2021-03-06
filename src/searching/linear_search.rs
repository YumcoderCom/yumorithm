/// A linear search or sequential search is a method for finding
/// an element within a collection. It sequentially checks each element of
/// the collection until a match is found or the whole collection has been searched.
///
/// # Examples
///
/// ```
/// use yumorithm::searching;
///
/// assert_eq!(searching::linear_search(&10, &vec![1, 2, 3]), None);
/// assert_eq!(searching::linear_search(&1, &vec![1, 2, 3]), Some(0));
/// ```
/// ┌──────────────────┐
/// │  -5 -2 0 7 9 10  │  search item = 7
/// └───┬──▲───────────┘
///     │  │
///     └──┘
///
/// ┌──────────────────┐
/// │  -5 -2 0 7 9 10  │
/// └──────┬─▲─────────┘
///        │ │
///        └─┘
///
/// ┌──────────────────┐
/// │  -5 -2 0 7 9 10  │
/// └────────┬─▲───────┘
///          │ │
///          └─┘
pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    // All inputs should be T that implemented PartialEq trait for
    // equality comparisons (`x == y` and `x != y`)
    // create an iterator that returns (index, data)
    //
    // # Examples
    //
    // ```
    // let a = ['a', 'b', 'c'];
    //
    // let mut iter = a.iter().enumerate();
    //
    // assert_eq!(iter.next(), Some((0, &'a')));
    // assert_eq!(iter.next(), Some((1, &'b')));
    // assert_eq!(iter.next(), Some((2, &'c')));
    // assert_eq!(iter.next(), None);
    // ```
    for (i, data) in arr.iter().enumerate() {
        if data == item {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_strings_test() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn linear_search_ints_test() {
        struct Tests {
            ds: Vec<i32>,
            cases: Vec<(i32, Option<usize>)>,
        }
        let tests = Tests {
            ds: vec![1, 2, 3, 4],
            cases: vec![
                (1, Some(0)), // (input, wanted)
                (2, Some(1)),
                (3, Some(2)),
                (4, Some(3)),
            ],
        };

        for t in tests.cases.iter() {
            let index = linear_search(&t.0, &tests.ds); // t.0: input
            assert_eq!(index, t.1); // t.1 expected/wanted
        }
    }

    #[test]
    fn linear_search_not_found_test() {
        let index = linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn linear_search_empty_test() {
        let index = linear_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}

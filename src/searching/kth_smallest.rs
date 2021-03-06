use std::cmp::Ordering;

/// find the kth smallest item in given array.
/// This approach is similar to the quick sort algorithm where we use the partition
/// on the input array recursively. But unlike quicksort, which processes both sides of
/// the array recursively, this algorithm works on only one side of the partition.
/// We recur for either the left or right side according to the position of pivot.
/// see also https://afteracademy.com/blog/kth-smallest-element-in-an-array
///
/// # Examples
///
/// ```
/// use yumorithm::searching;
///
/// assert_eq!(searching::kth_smallest(10, &mut vec![1, 2, 3]), None);
/// assert_eq!(searching::kth_smallest(2, &mut vec![101, 20, 305]), Some(101));
/// ```
pub fn kth_smallest<T: PartialOrd + Copy>(k: usize, arr: &mut [T]) -> Option<T> {
    if arr.is_empty() || k > arr.len() {
        return None;
    }

    Some(find_kth_smallest(arr, k, 0, arr.len()))
}

/* private */
fn find_kth_smallest<T>(arr: &mut [T], k: usize, left: usize, right: usize) -> T
where
    T: PartialOrd + Copy,
{
    if left == right {
        return arr[left];
    }
    let pivot = partition(arr, left, right);
    let i = pivot - left + 1;
    match k.cmp(&i) {
        Ordering::Equal => arr[pivot],
        Ordering::Less => find_kth_smallest(arr, k, left, pivot - 1),
        Ordering::Greater => find_kth_smallest(arr, k - i, pivot + 1, right),
    }
}

/// Partition the array A[left .. right] into two subarrays
/// A[left .. pos] and A[pos + 1 .. right]
/// such that each element of A[left .. pos] is less than each element of A[pos + 1 .. right]
///
/// ┌──────────┬─┬──────────┐
/// │   <= x   │x│    >= x  │
/// └──────────┴─┴──────────┘
///
/// ┌────────────────────┐
/// │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
/// └─┬─┬────────────────┘
///   │ │
///   i j
///
/// ┌────────────────────┐
/// │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
/// └─┬────┬─────────────┘
///   │    │
///   i ─► j
///
/// ┌────────────────────┐
/// │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
/// └─┬───────┬──────────┘
///   │       │
///   i ────► j
///
///             ┌─────┐
///         ┌───▼─────▼──────────┐
///         │ 6 5 13 10 8 3 2 11 │      pivot=a[0]=6
///         └───┬─────┬──────────┘
///             │     │
///          ──►i     j
///
/// ┌────────────────────┐
/// │ 6 5 13 10 8 3 2 11 │              pivot=a[0]=6
/// └───┬───────┬────────┘
///     │       │
///  ──►i       j
///
///            ┌───────┐
///      ┌─────▼───────▼──────┐
///      │ 6 5 3 10 8 13 2 11 │         pivot=a[0]=6
///      └─────┬───────┬──────┘
///            │       │
///       ───► i       j
///
///              ┌───────┐
///      ┌───────▼───────▼────┐
///      │ 6 5 3 2 8 13 10 11 │         pivot=a[0]=6
///      └───────┬───────┬────┘
///              │       │
///       ─────► i       j
///
///
/// ┌────────────────────┐
/// │ 6 5 3 2 8 13 10 11 │              pivot=a[0]=6
/// └───────┬──────────┬─┘
///         │          │
///  ─────► i          j
///
///
///        ┌─────┐
///      ┌─▼─────▼────────────┐
///      │ 2 5 3 6 8 13 10 11 │        swap pivot and x
///      └───────┬──────────┬─┘
///              │          │
///       ─────► i          j
fn partition<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) -> usize {
    // let pivot = arr[right]; // if uncomment then T should be in the form of  T: PartialEq + Copy
    let mut i = left;
    for j in (left + 1)..right {
        if arr[j] <= arr[left] {
            i = i + 1;
            arr.swap(i, j)
        }
    }
    arr.swap(i, left);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_smallest_partition() {
        let mut arr = [6, 10, 13, 5, 8, 3, 2, 11];
        let len = arr.len() - 1;
        let index = partition(&mut arr, 0, len);

        assert_eq!(3, index); // return index of arr[3] = 6
    }

    #[test]
    fn kth_smallest_empty_test() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(1, &mut zero);

        assert_eq!(None, first);
    }

    #[test]
    fn kth_smallest_has_one_element_test() {
        let mut one = [1];
        let first = kth_smallest(1, &mut one);

        assert_eq!(Some(1), first); // or assert_eq!(1, first.unwrap());
    }

    #[test]
    fn kth_smallest_gt_len() {
        let mut arr = [1, 2, 3];
        let elm10th = kth_smallest(10, &mut arr);

        assert_eq!(None, elm10th);
    }

    #[test]
    fn kth_smallest_has_n_elements_test() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut arr = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(1, &mut arr);
        let third = kth_smallest(3, &mut arr);
        let sixth = kth_smallest(6, &mut arr);
        let fourteenth = kth_smallest(14, &mut arr);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}

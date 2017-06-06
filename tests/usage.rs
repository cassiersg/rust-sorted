#[macro_use]
extern crate sorted;

use sorted::*;

order_by_key!{ KeyFirstOrder:
    fn (K: Ord + Copy, T)(entry: (K,T)) -> K { entry.0 }
}

order_by_key!{ KeySecondOrder:
    fn (K: Ord + Copy, T)(entry: (T,K)) -> K { entry.1 }
}

#[test]
fn sorted_array() {
    let arr = [7,2,9,6];
    let v = DefaultOrder::by_sorting(arr);
    assert_eq!(
        *v.as_inner(),
        [2,6,7,9]
    );
}

#[test]
fn sorted_slice() {
    let mut arr = [3,2,4,1];
    let s = DefaultOrder::by_sorting(&mut arr[..]);
    assert_eq!(
        s.as_inner(),
        &[1,2,3,4]
    );
}

#[test]
fn sorted_vec() {
    let v = DefaultOrder::by_sorting(vec![4,3,1,2]);
    assert_eq!(
        v.as_slice(),
        &[1,2,3,4]
    );
}

#[test]
fn sort_by_first() {
    let s = vec![(5,3),(2,7),(3,4)];
    let v = KeyFirstOrder::by_sorting(s);
    assert_eq!(
        &[(2,7),(3,4),(5,3)],
        v.as_slice()
    );
}

#[test]
fn sort_by_second() {
    let s = vec![(5,3),(2,7),(3,4)];
    let v = KeySecondOrder::by_sorting(s);
    assert_eq!(
        &[(5,3),(3,4),(2,7)],
        v.as_slice()
    );
}

#[test]
fn sorted_slice_from_sorted_vec() {
    let vec = SortedVec::by_sorting(vec![4,9,2,33,1], DefaultOrder);
    let slice = SortedSlice::from(&vec);
    assert_eq!(
        [1,2,4,9,33][..],
        *slice
    );
}

#[test]
fn sorted_vec_from_sorted_slice() {
    let mut arr = [5,3,7,9];
    let slice = SortedSlice::by_sorting(&mut arr, DefaultOrder);
    let vec = SortedVec::from(slice);
    assert_eq!(
        [3,5,7,9],
        vec.as_slice()
    );
}

#[test]
fn take_sorted_iterator() {
    fn take_sorted<I>(sorted: I) where I: IntoIterator<Item=i32> + IsSorted {
        let v: Vec<_> = sorted.into_iter().collect();
        assert_eq!(
            vec![2,3,8,10],
            v
        );
    }
    let data: Vec<i32> = vec![3,8,2,10];
    let vec = SortedVec::by_sorting(data, DefaultOrder);
    take_sorted(vec);
}

#[test]
fn sorted_insert() {
    let mut vec = SortedVec::by_sorting(vec![4,8,2,0], DefaultOrder);
    vec.insert(6);
    assert_eq!(
        [0,2,4,6,8],
        vec.as_slice()
    );
}
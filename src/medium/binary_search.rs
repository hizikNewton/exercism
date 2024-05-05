pub mod binary_search {
    pub fn find(array: &[i32], key: i32) -> Option<usize> {
        if array.is_empty() {
            return None;
        }
        let mut arr = array;
        let mid = arr.len() / 2;
        let mut idx = mid;
        loop {
            if key < arr[0] {
                return None;
            } else if key > arr[arr.len() - 1] {
                return None;
            } else if arr[0] == key {
                return Some(0);
            } else {
                if arr[mid] == key {
                    return Some(idx);
                } else if arr[mid - 1] == key {
                    return Some(idx - 1);
                } else if key < mid as i32 {
                    arr = &arr[0..=mid];
                } else {
                    println!("{key:?},{mid:?}");
                    arr = &arr[mid..];
                }
                idx += mid;
            }
        }
    }
}

// The &[] borrows are required for the base exercise,
// where `find` is not generic. Once `find` is made generic,
// the borrows become needless. Since we want the tests to work
// without clippy warnings for both people who take on the
// additional challenge and people who don't, we disable this lint.
//#![allow(clippy::needless_borrows_for_generic_args)]
use binary_search::*;
#[test]
pub fn finds_a_value_in_an_array_with_one_element() {
    assert_eq!(find(&[6], 6), Some(0));
}
#[test]
fn finds_first_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 1), Some(0));
}
#[test]
fn finds_second_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 2), Some(1));
}
#[test]
fn finds_a_value_in_the_middle_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
}
#[test]
fn finds_a_value_at_the_beginning_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
}
#[test]
fn finds_a_value_at_the_end_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
}
#[test]
fn finds_a_value_in_an_array_of_odd_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
}
#[test]
fn finds_a_value_in_an_array_of_even_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
}
#[test]
fn identifies_that_a_value_is_not_included_in_the_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 7), None);
}
#[test]
fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
}
#[test]
fn a_value_larger_than_the_arrays_largest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 13), None);
}
#[test]
fn nothing_is_included_in_an_empty_array() {
    assert_eq!(find(&[], 1), None);
}
#[test]
fn nothing_is_found_when_the_left_and_right_bounds_cross() {
    assert_eq!(find(&[1, 2], 0), None);
} /*
  #[test]
  #[cfg(feature = "generic")]
  fn works_for_arrays() {
      assert_eq!(find([6], 6), Some(0));
  }
  #[test]
  #[cfg(feature = "generic")]
  fn works_for_vec() {
      let vector = vec![6];
      assert_eq!(find(&vector, 6), Some(0));
      assert_eq!(find(vector, 6), Some(0));
  }
  #[test]
  #[cfg(feature = "generic")]
  fn works_for_str_elements() {
      assert_eq!(find(["a"], "a"), Some(0));
      assert_eq!(find(["a", "b"], "b"), Some(1));
  } */

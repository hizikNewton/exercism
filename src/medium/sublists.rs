use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq+Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    let val = (first_list,second_list);

    if val.0 == val.1{
        Comparison::Equal

    }else if val.1.len()>val.0.len() {
        if val.0.is_empty(){Comparison::Sublist}
        else{
        let possible_idx:Vec<(usize, &T)> = val.1.iter().enumerate().filter(|(_,ele)| {
            val.0[0] == **ele}).collect();
        for (idx,_) in possible_idx{
            if val.1[idx..val.0.len()+idx] == *val.0{
                return Comparison::Sublist;
            }else{
                continue;
            }
        }
        return Comparison::Unequal;
        }
    }
    else if val.0.len() > val.1.len()  {
        if val.1.is_empty(){Comparison::Superlist}
        else{
        let possible_idx:Vec<(usize, &T)> = val.0.iter().enumerate().filter(|(_,ele)| {
            val.1[0] == **ele}).collect();
        for (idx,_) in possible_idx{
            if val.0[idx..val.1.len()+idx] == *val.1{
                return Comparison::Superlist;
            }else{
                continue;
            }
        }
        return Comparison::Unequal;
        }
    }
    else {
        Comparison::Unequal
    }
}

#[test]
fn empty_lists() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}
#[test]
fn empty_list_within_non_empty_list() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn non_empty_list_contains_empty_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}
#[test]
fn list_equals_itself() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}
#[test]
fn different_lists() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[2, 3, 4];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
#[test]
fn false_start() {
    let list_one: &[i32] = &[1, 2, 5];
    let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn consecutive() {
    let list_one: &[i32] = &[1, 1, 2];
    let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn sublist_at_start() {
    let list_one: &[i32] = &[0, 1, 2];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn sublist_in_middle() {
    let list_one: &[i32] = &[2, 3, 4];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn sublist_at_end() {
    let list_one: &[i32] = &[3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
#[test]
fn at_start_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}
#[test]
fn in_middle_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}
#[test]
fn at_end_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}
#[test]
fn first_list_missing_element_from_second_list() {
    let list_one: &[i32] = &[1, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
#[test]
fn second_list_missing_element_from_first_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[1, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
#[test]
fn first_list_missing_additional_digits_from_second_list() {
    let list_one: &[i32] = &[1, 2];
    let list_two: &[i32] = &[1, 22];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
#[test]
fn order_matters_to_a_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[3, 2, 1];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
#[test]
fn same_digits_but_different_numbers() {
    let list_one: &[i32] = &[1, 0, 1];
    let list_two: &[i32] = &[10, 1];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

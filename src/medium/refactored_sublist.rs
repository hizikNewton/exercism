use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(sub: &[T], super_: &[T]) -> bool {
    if sub.is_empty() {
        return true;
    }

    for window in super_.windows(sub.len()) {
        if window == sub {
            return true;
        }
    }

    false
}
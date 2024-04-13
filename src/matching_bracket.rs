pub mod matching_brackets {

    use std::collections::HashMap;
    ///Instructions
    ///Given a string containing brackets [], braces {}, parentheses (), or any combination thereof, verify that any and all pairs are matched and nested correctly. The string may also contain other characters, which for the purposes of this exercise should be ignored.

    pub fn brackets_are_balanced(string: &str) -> bool {
        let mut pair_map = HashMap::new();
        let pairs: [(char, char); 3] = [('[', ']'), ('{', '}'), ('(', ')')];
        for (lb, rb) in &pairs {
            pair_map.insert(rb, (lb, rb));
        }
        let mut string_char: Vec<char> = string.chars().collect();
        let legit:Vec<char>  =  pairs.iter().flat_map(|&(a,b)| vec![a,b]).collect();
        string_char = string_char.iter().cloned().filter(|ch| legit.contains(ch)).collect();
        let mut rem = string_char.clone();

        loop {
            if string_char.is_empty() {
                return true;
            }
            'inner: {
                for (i, j) in string_char.iter().enumerate() {
                    match pair_map.get(j) {
                        Some(v) => {
                            if (i as i8 != 0) && string_char[i - 1] == *v.0 {
                                rem.drain((i - 1)..i + 1);
                                string_char = rem.clone();
                                break 'inner;
                            } else {
                                return false;
                            }
                        }
                        None => {
                            if (i == string_char.len() - 1) && (!string_char.is_empty()) {
                                return false;
                            }
                        }
                    }
                }
                return true;
            }
        }
    }

    
}

use matching_brackets::brackets_are_balanced;
#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}
#[test]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}
#[test]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}
#[test]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}
#[test]
fn wrong_closing_bracket() {
    assert!(!brackets_are_balanced("{]"));
}
#[test]
fn paired_with_whitespace() {
    assert!(brackets_are_balanced("{ }"));
}
#[test]
fn partially_paired_brackets() {
    assert!(!brackets_are_balanced("{[])"));
}
#[test]
fn simple_nested_brackets() {
    assert!(brackets_are_balanced("{[]}"));
}
#[test]
fn several_paired_brackets() {
    assert!(brackets_are_balanced("{}[]"));
}
#[test]
fn paired_and_nested_brackets() {
    assert!(brackets_are_balanced("([{}({}[])])"));
}
#[test]
fn unopened_closing_brackets() {
    assert!(!brackets_are_balanced("{[)][]}"));
}
#[test]
fn unpaired_and_nested_brackets() {
    assert!(!brackets_are_balanced("([{])"));
}
#[test]
fn paired_and_wrong_nested_brackets() {
    assert!(!brackets_are_balanced("[({]})"));
}
#[test]
fn paired_and_incomplete_brackets() {
    assert!(!brackets_are_balanced("{}["));
}
#[test]
fn too_many_closing_brackets() {
    assert!(!brackets_are_balanced("[]]"));
}
#[test]
fn early_incomplete_brackets() {
    assert!(!brackets_are_balanced(")()"));
}
#[test]
fn early_mismatched_brackets() {
    assert!(!brackets_are_balanced("{)()"));
}
#[test]
fn math_expression() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}
#[test]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(brackets_are_balanced(input));
}

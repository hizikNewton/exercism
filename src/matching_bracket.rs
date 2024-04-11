pub mod matching_brackets{

use std::collections::HashMap;
///Instructions
///Given a string containing brackets [], braces {}, parentheses (), or any combination thereof, verify that any and all pairs are matched and nested correctly. The string may also contain other characters, which for the purposes of this exercise should be ignored.

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut pair_map = HashMap::new();
    let pairs:[(char,char);3] = [('[',']'),('{','}'),('(',')')];
    for (lb,rb) in &pairs{
        pair_map.insert(lb, (lb,rb));
    }
    let string_char:Vec<char> = string.chars().collect();
    let length = string_char.len();
    if length%2==0{
        let mut midpoint = length/2;
        while midpoint>0 {
            let (lp,rp) = (string_char[midpoint-1],string_char[midpoint]);
            print!("lp,rp is {}{}",lp,rp);
            match pair_map.get(&lp){
                    Some(val)=>{
                        if *val.0==lp && *val.1==rp{
                            midpoint=midpoint-1;
                        }else{
                            return  false;
                        }
                    },
                    None=> {
                       return false;
                    }
                };
        };
        return true;
    }
    false
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

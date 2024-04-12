pub mod matching_brackets{

use std::collections::HashMap;
///Instructions
///Given a string containing brackets [], braces {}, parentheses (), or any combination thereof, verify that any and all pairs are matched and nested correctly. The string may also contain other characters, which for the purposes of this exercise should be ignored.

static mut balanced:Vec<bool> = Vec::new();
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut pair_map = HashMap::new();
    let pairs:[(char,char);3] = [('[',']'),('{','}'),('(',')')];
    for (lb,rb) in &pairs{
        pair_map.insert(lb, (lb,rb));
    }
    let string_char:Vec<char> = string.chars().collect();
    let length = string_char.len();
    if length%2==0{
        for (i,j) in string_char.iter().enumerate(){
            match pair_map.get(j) {
                Some(val)=>{
                    let position = string_char.iter().position(|c| c==val.1);
                    match position {
                        Some(chIndex)=>{
                            let subslice = &string_char[i+1..chIndex];
                            if subslice.len()!=0{
                                let substring:String = subslice.iter().collect();
                                println!("{substring:?}");
                                brackets_are_balanced(&substring);
                            }else{
                                balanced.push(true);
                                if chIndex<length{
                                    let remslice = &string_char[chIndex+1..];
                                    let supstring:String = remslice.iter().collect();
                                    brackets_are_balanced(&supstring);
                                }
                            }
                        },
                        None=> {return false}
                        }
                },
                None=>{return false}
            }
        }
       /*  let mut midpoint = length/2;
        while midpoint>0 {
            let (lp,rp) = (string_char[midpoint-1],string_char[midpoint+1]);
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
        }; */
        print!()
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

/* 
use bob::bob_risky;
use kg_garden::kindergarten_garden::plants;
use matching_bracket::matching_brackets;
use easy::sum_of_multiple::sum_of_multiples;
use medium::{anagram::anagrams_for};
*/

pub mod medium;
use medium::sublists;
fn main() {

  let list_one: &[i32] = &[1, 1, 2];
  let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
  let output = sublists::sublist(list_one, list_two);
  print!("output yeahh {:?}",output);
  
}


  /*
    let res = Clock::new(1, -40).to_string();
    print!("result is {res:?}");
    let res = sum_of_multiples::sum_of_multiples(1,&[0]);
    let res = matching_brackets::brackets_are_balanced("{ }");
    print!("result is {res:?}");
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    println!("whattt {:?}",plants(diagram, student));
     let prime_factors = prime_factors::factors(93819012551);
    println!("{:?}",prime_factors);
    println!("{:?}",bob_risky::reply("1, 2, 3 GO!"));*/
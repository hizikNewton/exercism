use std::collections::HashMap;

pub mod alphametics{
    use std::{collections::HashMap, result};
    pub fn solve(input: &str) ->  Option<HashMap<char, u8>>{
        let mut input_map:HashMap<char, Option<u32>> = HashMap::new();

        let input =input.to_string();
        for i in input.chars(){
            if i.is_alphabetic(){
            input_map.insert(i, None);
            }
        }
        let values = input.split("==").collect::<Vec<&str>>();
        let ops = values[0].to_string();
        let opv = ops.split("+").map(|i|i.trim()).collect::<Vec<&str>>();
        let result = values[1].trim();

        for i in opv.chunks(2){
            match i{
                [val1,val2]=>deal(val1,val2,result,&mut input_map),
                [last_val]=>print!("{:?}---",last_val),
                _=>()
            }
        }
        
        None
    }

    pub fn deal(op1:&str,op2:&str,result:&str,input_map:&mut HashMap<char, Option<u32>> ){
        let res = result.chars().collect::<Vec<char>>(); 
        let op1 = op1.chars().collect::<Vec<char>>();
        let op2 = op2.chars().collect::<Vec<char>>(); 
        let mut boxed:Vec<Vec<char>> = Vec::new();

        let result_len = res.len();
        
        if result_len>op1.len()||res.len()>op2.len(){
            let mut box_op1 = vec!['#';result_len-op1.len()];
            let mut box_op2 = vec!['#';result_len-op2.len()];
            box_op1.extend_from_slice(&op1);
            box_op2.extend_from_slice(&op2);
            //there is carryover
            input_map.insert(res[0], Some(1));

            boxed = vec![
                box_op1,
                box_op2,
                res
            ];
            
           
        print!("your map is{:?}",input_map);
        }


    }

    pub fn do_replacement(boxed:Vec<Vec<char>>,input_map:&mut HashMap<char, Option<u32>> ){
        for key in input_map.keys().collect::<Vec<char>>(){
            for row in boxed{

            }
        }
    }
}

pub fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
    let answer = alphametics::solve(puzzle);
    let solution: HashMap<char, u8> = solution.iter().cloned().collect();
    assert_eq!(answer, Some(solution));
}
#[test]
fn with_three_letters() {
    assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
}
#[test]
#[ignore]
fn must_have_unique_value_for_each_letter() {
    let answer = alphametics::solve("A == B");
    assert_eq!(answer, None);
}
#[test]
#[ignore]
fn leading_zero_solution_is_invalid() {
    let answer = alphametics::solve("ACA + DD == BD");
    assert_eq!(answer, None);
}
#[test]
#[ignore]
fn sum_must_be_wide_enough() {
    let answer = alphametics::solve("ABC + DEF == GH");
    assert_eq!(answer, None);
}
#[test]
#[ignore]
fn puzzle_with_two_digits_final_carry() {
    assert_alphametic_solution_eq(
        "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
        &[('A', 9), ('B', 1), ('C', 0)],
    );
}
#[test]
#[ignore]
fn puzzle_with_four_letters() {
    assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
}
#[test]
#[ignore]
fn puzzle_with_six_letters() {
    assert_alphametic_solution_eq(
        "NO + NO + TOO == LATE",
        &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
    );
}
#[test]
#[ignore]
fn puzzle_with_seven_letters() {
    assert_alphametic_solution_eq(
        "HE + SEES + THE == LIGHT",
        &[
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ],
    );
}
#[test]
#[ignore]
fn puzzle_with_eight_letters() {
    assert_alphametic_solution_eq(
        "SEND + MORE == MONEY",
        &[
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ],
    );
}
#[test]
#[ignore]
fn puzzle_with_ten_letters() {
    assert_alphametic_solution_eq(
        "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
        &[
            ('A', 5),
            ('D', 3),
            ('E', 4),
            ('F', 7),
            ('G', 8),
            ('N', 0),
            ('O', 2),
            ('R', 1),
            ('S', 6),
            ('T', 9),
        ],
    );
}
#[test]
#[ignore]
fn puzzle_with_ten_letters_and_199_addends() {
    assert_alphametic_solution_eq(
        "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        &[
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ],
    );
}
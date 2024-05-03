use std::collections::HashMap;

pub mod alphametics {
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct PossibleValue {
        values: Vec<u32>,
    }
    impl PossibleValue {
        fn remove(&mut self, number: &[u32]) {
            for i in number {
                if let Some(idx) = self.values.iter_mut().position(|x| x == i) {
                    self.values.remove(idx);
                }
            }
        }
    }
    pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
        let mut pv = PossibleValue {
            values: (0..=9).collect::<Vec<_>>(),
        };
        let input = input.to_string();
        // input.chars().filter(|ch| ch.is_alphabetic());
        let values = input.split("==").collect::<Vec<&str>>();
        let ops = values[0].to_string();
        let opv = ops.split("+").map(|i| i.trim()).collect::<Vec<&str>>();
        let result = values[1].trim();

        deal(opv, result, &mut pv);

        None
    }

    pub fn deal(operating_values: Vec<&str>, result: &str, possible_value: &mut PossibleValue) {
        let mut boxed: Vec<Vec<char>> = Vec::new();
        let res = result.chars().collect::<Vec<char>>();
        let max_opv = *operating_values.iter().max_by_key(|x| x.len()).unwrap();
        let total_len = max_opv.len();
        operating_values
            .iter()
            .for_each(|i| boxed.push(i.chars().collect()));

        let result_len = res.len();

        if result_len > total_len {
            for row in boxed.iter_mut() {
                let mut box_op1 = vec!['#'; result_len - row.len()];
                box_op1.extend_from_slice(&row);
                *row = box_op1;
            }

            boxed.push(res.clone());
            let mut carry_over: bool = false;
            for i in 0..result_len {
                //get second column
                let next_col = boxed
                    .iter()
                    .map(|row| row[i])
                    .filter(|ch| *ch != '#')
                    .collect::<Vec<char>>();

                if let Some((nc_rv, nc_opv)) = next_col.split_last() {
                    match nc_opv {
                        //no operating value
                        [] => {
                            do_replacement(&mut boxed, vec![(nc_rv, &'1')]);

                            possible_value.remove(&[1])
                        }

                        _ => {
                            println!("{:?}--", nc_opv);
                            if nc_opv.contains(&'0') {
                                carry_over = true
                            }
                            loop {
                                if let Some(result) = generate_result(&possible_value.values) {
                                    let guess = guess(
                                        possible_value,
                                        nc_opv.to_vec(),
                                        (*nc_rv, *result),
                                        carry_over,
                                        &mut boxed,
                                    );
                                    if guess.is_ok() {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    };
                }
            }
            println!("{boxed:?}");
        }
    }

    pub fn do_replacement(boxed: &mut Vec<Vec<char>>, replacement: Vec<(&char, &char)>) {
        for (key, value) in replacement.iter() {
            for row in boxed.iter_mut() {
                if row.contains(key) {
                    row.iter_mut().for_each(|i| {
                        if i == *key {
                            *i = **value;
                        };
                    });
                }
            }
        }
    }

    pub fn generate_result<'a>(possible_value: &'a Vec<u32>) -> Option<&u32> {
        possible_value.iter().next()
    }

    pub fn guess(
        possible_value: &mut PossibleValue,
        operating_values: Vec<char>,
        result_value: (char, u32),
        carry_over: bool,
        boxed: &mut Vec<Vec<char>>,
    ) -> Result<(), String> {
        //remove(vec![result_value.1].as_slice(), possible_value);
        possible_value.remove(&[result_value.1]);
        let unknown: Vec<&char> = operating_values
            .iter()
            .filter(|ch| ch.is_alphabetic())
            .collect();
        let known: Vec<char> = operating_values
            .iter()
            .filter(|ch| ch.is_digit(10))
            .cloned()
            .collect();
        let cpo = possible_value.values.clone();
        let mut trial_val = cpo.windows(unknown.len());

        loop {
            let trial_v = trial_val.next();

            if let Some(tv) = trial_v {
                let mut chs: Vec<char> = tv
                    .iter()
                    .map(|tv| char::from_digit(*tv, 10).unwrap())
                    .collect();
                chs.extend_from_slice(known.as_slice());

                let mut sum_of_opv = chs
                    .iter()
                    .fold(0, |acc, ch: &char| acc + ch.to_digit(10).unwrap());
                if carry_over {
                    sum_of_opv += 1;
                }

                if sum_of_opv % 10 == result_value.1 {
                    let mut iter: Vec<(&char, &char)> =
                        operating_values.iter().zip(chs.iter()).collect();
                    let res_char = char::from_digit(result_value.1, 10).unwrap();
                    iter.push((&result_value.0, &res_char));

                    do_replacement(boxed, iter);
                    possible_value.remove(tv);
                    // remove(tv, possible_value);
                    return Ok(());
                }
            } else {
                return Err("".to_string());
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

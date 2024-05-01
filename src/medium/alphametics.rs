use std::collections::HashMap;

pub mod alphametics {
    use std::collections::HashMap;
    enum result_range {
        GreaterThan10,
        LessThan10,
        GreaterOrLesser,
    }
    pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
        let mut input_map: HashMap<char, Vec<u32>> = HashMap::new();

        let input = input.to_string();
        // input.chars().filter(|ch| ch.is_alphabetic());
        input_map.insert('#', (0..=9).collect());
        let values = input.split("==").collect::<Vec<&str>>();
        let ops = values[0].to_string();
        let opv = ops.split("+").map(|i| i.trim()).collect::<Vec<&str>>();
        let result = values[1].trim();

        deal(opv, result, &mut input_map);

        None
    }

    pub fn deal(
        operating_values: Vec<&str>,
        result: &str,
        possible_value: &mut HashMap<char, Vec<u32>>,
    ) {
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
                    let mut sum: &char;

                    match nc_opv {
                        //no operating value
                        [] => {
                            carry_over = true;
                            sum = &'1';
                            do_replacement(&mut boxed, vec![(nc_rv, sum)])
                        }
                        //just one operating value
                        [just_one] => {
                            print!("next column{just_one:?}");
                            /*
                            if just_one != *nc_rv && nc_rv.is_alphabetic()
                            let opv: Vec<char> = nc_opv
                            .iter()
                            .filter_map(|x| x.is_alphabetic().then(|| *x))
                            .collect();
                            if i == 1 {
                                do_replacement(&mut boxed, vec![(&just_one, &'9'), (nc_rv, &'0')]);
                                remove(&[0, 9], possible_value)
                            } else {
                                let guess =
                                    guess(possible_value, nc_opv.to_vec(), nc_rv, &mut boxed);
                            }
                            [just_one] if !nc_rv.is_alphabetic() => {
                                println!("next column operating value{nc_opv:?}");
                                println!(
                                    "my guess for {:?}----{:?}",
                                    just_one,
                                    guess(possible_value, nc_opv.to_vec(), nc_rv, &mut boxed)
                                )
                            } */
                        }

                        //more than one operating value
                        _ => {
                            if nc_rv.is_alphabetic() {
                                if let Some(result) = generate_result(carry_over) {
                                    let result = result % 10;
                                    let pred =
                                        guess(possible_value, nc_opv.to_vec(), result, &mut boxed);
                                    println!("predicted{pred:?}");
                                }
                            }
                        }
                    };
                }
            }
        }

        println!("{boxed:?}");
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

    pub fn remove(number: &[u32], possible_value: &mut HashMap<char, Vec<u32>>) {
        let po = possible_value.get_mut(&'#').unwrap();
        for i in number {
            let idx = po.iter_mut().position(|x| x == i).unwrap();
            po.remove(idx);
        }
    }

    pub fn generate_result(carry_over: bool) -> Option<u32> {
        if carry_over {
            (10..19).next()
        } else {
            (1..19).next()
        }
    }

    pub fn guess(
        possible_value: &mut HashMap<char, Vec<u32>>,
        operating_values: Vec<char>,
        result_value: u32,
        boxed: &mut Vec<Vec<char>>,
    ) -> Option<Vec<u32>> {
        remove(vec![result_value].as_slice(), possible_value);
        let po = possible_value.clone();
        let cpo = po.get(&'#').unwrap().as_slice();

        let unknown: Vec<&char> = operating_values
            .iter()
            .filter(|ch| ch.is_alphabetic())
            .collect();
        let known: Vec<&char> = operating_values
            .iter()
            .filter(|ch| ch.is_digit(10))
            .collect();

        let mut trial_val = cpo.windows(unknown.len());

        loop {
            let trial_v = trial_val.next();

            if let Some(tv) = trial_v {
                let chs = tv
                    .iter()
                    .map(|tv| char::from_digit(*tv, 10).unwrap())
                    .collect::<Vec<_>>();
                chs.extend_from_slice(known);

                let sum_of_opv = tv.iter().fold(0, |acc, ch| acc + ch);

                if sum_of_opv == result_value {
                    let iter: Vec<(&char, &char)> =
                        operating_values.iter().zip(chs.iter()).collect();
                    do_replacement(boxed, iter);
                    remove(tv, possible_value);
                    return Some(tv.to_vec());
                }
            } else {
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

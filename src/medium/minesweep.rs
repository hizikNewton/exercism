pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<String> = Vec::new();
    let mut minefields:Vec<&[&str]>= minefield.windows(2).collect();
    let last_item = [*minefield.last().unwrap()];
    minefields.push(&last_item);

    for (pos, j) in minefields.iter().enumerate() {
        let prev = if pos > 0 { Some(minefield[pos - 1])} else {None};
        match j {
            [current, next] => result.push(annotate_row(prev, current, Some(next))),
            [current] => result.push(annotate_row(prev, current, None)),
           _=>()
        };
    } 
    result
}

pub fn annotate_row(prev: Option<&str>, current: &str, next: Option<&str>) -> String {
    let mut current_row = current.chars().collect::<Vec<char>>();
    let current_row_dup = current_row.clone();

    for (i, ch) in current_row.iter_mut().enumerate() {
        let mut count:u32 = 0;
        if *ch == ' ' {
            for ele in [prev,Some(current),next] {
                if let Some(ele) = ele {
                    let item = ele.chars().collect::<Vec<char>>();
                    
                     if item[i] == '*' && item!=current_row_dup {
                        count += 1;
                    }
                    if i!=item.len()-1 && item[i + 1] == '*' {
                        count += 1;
                    }
                    if i!=0 && item[i - 1] == '*' {
                        count += 1
                    }
                }
            }
        }
        if count!=0{
            *ch = char::from_digit(count, 10).unwrap();
        }
    }
    current_row.iter().collect::<String>()
}

pub fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}
pub fn remove_annotations_in_row(row: &str) -> String {
    row.as_bytes()
        .iter()
        .map(|&ch| match ch {
            b'*' => '*',
            _ => ' ',
        })
        .collect()
}
pub fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}
#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}
#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}
#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}
#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}
#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}
#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}
#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}
#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}
#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}
#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}
#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}
#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}

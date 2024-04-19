pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let x:Vec<String> = Vec::new();
    if minefield.is_empty(){
        return x;
    }
    let mut count = 0;
    let result:Vec<String> = Vec::new();
    for (i,j) in minefield.windows(2).enumerate(){
        if i!=0{
            let prev = minefield[i-1];
            let prev_row = prev.chars().collect::<Vec<char>>();
            if let [ current,next] = j{
               let current_row = current.chars().collect::<Vec<char>>();
               let mut c_row_dup = current_row.clone();
               let next_row = next.chars().collect::<Vec<char>>();
                for (i,ch) in current_row.iter().enumerate(){
                    if *ch==' '{
                        if current_row[i+1]=='*'{
                            count+=1;
                        }
                        if current_row[i-1]=='*'{
                            count+=1;
                        }
                        if next_row[i+1]=='*'{
                            count+=1;
                        }
                        if next_row[i-1]=='*'{
                            count+=1;
                        }if prev_row[i+1]=='*'{
                            count+=1;
                        }
                        if prev_row[i-1]=='*'{
                            count+=1;
                        }
                    }else{
                        continue;
                    }
                   c_row_dup[i] = char::from_u32(count).expect("were panick");
                }
                c_row_dup
            }

        }else{

        }

    }
    return x;
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

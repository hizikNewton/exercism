mod proverb{
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb:Vec<String> = Vec::new();
    for (idx,elem) in list.iter().enumerate(){
        if(idx+1)!=list.len(){
            proverb.push(produce((elem,list[idx+1])));
        }else{
            proverb.push(format!("And all for the want of a {}.",list[0]));
        }
    }
    proverb.join("\n")
}

pub fn produce(items:(&str,&str))-> String{
    format!("For want of a {0} the {1} was lost.",items.0,items.1)
}

}
#[test]
fn zero_pieces() {
    let input = &[];
    let output = proverb::build_proverb(input);
    let expected = String::new();
    assert_eq!(output, expected);
}
#[test]
fn one_piece() {
    let input = &["nail"];
    let output = proverb::build_proverb(input);
    let expected: String = ["And all for the want of a nail."].join("\n");
    assert_eq!(output, expected);
}
#[test]
fn two_pieces() {
    let input = &["nail", "shoe"];
    let output = proverb::build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
#[test]
fn three_pieces() {
    let input = &["nail", "shoe", "horse"];
    let output = proverb::build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
#[test]
fn full_proverb() {
    let input = &[
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let output = proverb::build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "For want of a horse the rider was lost.",
        "For want of a rider the message was lost.",
        "For want of a message the battle was lost.",
        "For want of a battle the kingdom was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
#[test]
fn four_pieces_modernized() {
    let input = &["pin", "gun", "soldier", "battle"];
    let output = proverb::build_proverb(input);
    let expected: String = [
        "For want of a pin the gun was lost.",
        "For want of a gun the soldier was lost.",
        "For want of a soldier the battle was lost.",
        "And all for the want of a pin.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

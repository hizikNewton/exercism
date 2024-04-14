/* 
Instructions
Your task is to determine what Bob will reply to someone when they say something to him or ask him a question.

Bob only ever answers one of five things:

"Sure." This is his response if you ask him a question, such as "How are you?" The convention used for questions is that it ends with a question mark.
"Whoa, chill out!" This is his answer if you YELL AT HIM. The convention used for yelling is ALL CAPITAL LETTERS.
"Calm down, I know what I'm doing!" This is what he says if you yell a question at him.
"Fine. Be that way!" This is how he responds to silence. The convention used for silence is nothing, or various combinations of whitespace characters.
"Whatever." This is what he answers to anything else.
 */
pub mod bob_risky {

struct ReplyPattern{
    question:bool,
    all_upper:bool,
    yell_question:bool,
    silence:bool
}
pub fn reply(message: &str) -> &str {
    let last = message[..].trim().to_string().pop().unwrap_or_else(|| ' ' );
    let stripped = message.chars().filter(|x|x.is_ascii_alphabetic()).collect::<Vec<_>>();
    
    let all_upper = if stripped.is_empty(){
        false}else{
        stripped.clone().into_iter().filter(|x|x.is_ascii_uppercase() ).collect::<Vec<_>>().len()==stripped.len()
        };
    let question = last=='?';
    let silence = last==' ';
    let yell_question = all_upper & question;
    

    let bob_reply = ReplyPattern{
        question:question,
        all_upper:all_upper,
        yell_question:yell_question,
        silence:silence
    };
   
    let resp = match bob_reply{
        ReplyPattern{yell_question:true,..}  =>"Calm down, I know what I'm doing!",
        ReplyPattern{question:true,..} =>"Sure.",
        ReplyPattern{silence:true,..} =>"Fine. Be that way!", 
        ReplyPattern{all_upper:true,..} =>"Whoa, chill out!",
        _ => "Whatever."
        

    };  
    resp
}
}

fn process_response_case(phrase: &str, expected_response: &str) {
    assert_eq!(bob_risky::reply(phrase), expected_response);
}
#[test]
/// stating something
fn stating_something() {
    process_response_case("Tom-ay-to, tom-aaaah-to.", "Whatever.");
}
#[test]
/// ending with whitespace
fn ending_with_whitespace() {
    process_response_case("Okay if like my  spacebar  quite a bit?   ", "Sure.");
}
#[test]
/// shouting numbers
fn shouting_numbers() {
    process_response_case("1, 2, 3 GO!", "Whoa, chill out!");
}
#[test]
/// other whitespace
fn other_whitespace() {
    process_response_case("\r\r 	", "Fine. Be that way!");
}
#[test]
/// shouting with special characters
fn shouting_with_special_characters() {
    process_response_case(
        "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!",
        "Whoa, chill out!",
    );
}
#[test]
/// talking forcefully
fn talking_forcefully() {
    process_response_case("Hi there!", "Whatever.");
}
#[test]
/// prattling on
fn prattling_on() {
    process_response_case("Wait! Hang on. Are you going to be OK?", "Sure.");
}
#[test]
/// forceful question
fn forceful_question() {
    process_response_case("WHAT'S GOING ON?", "Calm down, I know what I'm doing!");
}
#[test]
/// shouting with no exclamation mark
fn shouting_with_no_exclamation_mark() {
    process_response_case("I HATE THE DENTIST", "Whoa, chill out!");
}
#[test]
/// asking gibberish
fn asking_gibberish() {
    process_response_case("fffbbcbeab?", "Sure.");
}
#[test]
/// question with no letters
fn question_with_no_letters() {
    process_response_case("4?", "Sure.");
}
#[test]
/// no letters
fn no_letters() {
    process_response_case("1, 2, 3", "Whatever.");
}
#[test]
/// statement containing question mark
fn statement_containing_question_mark() {
    process_response_case("Ending with ? means a question.", "Whatever.");
}
//NEW
#[test]
/// multiple line question
fn multiple_line_question() {
    process_response_case(
        "\rDoes this cryogenic chamber make me look fat?\rNo.",
        "Whatever.",
    );
}
#[test]
/// non-question ending with whitespace
fn nonquestion_ending_with_whitespace() {
    process_response_case(
        "This is a statement ending with whitespace      ",
        "Whatever.",
    );
}
#[test]
/// shouting
fn shouting() {
    process_response_case("WATCH OUT!", "Whoa, chill out!");
}
#[test]
/// non-letters with question
fn nonletters_with_question() {
    process_response_case(":) ?", "Sure.");
}
#[test]
/// shouting gibberish
fn shouting_gibberish() {
    process_response_case("FCECDFCAAB", "Whoa, chill out!");
}
#[test]
/// asking a question
fn asking_a_question() {
    process_response_case("Does this cryogenic chamber make me look fat?", "Sure.");
}
#[test]
/// asking a numeric question
fn asking_a_numeric_question() {
    process_response_case("You are, what, like 15?", "Sure.");
}
#[test]
/// silence
fn silence() {
    process_response_case("", "Fine. Be that way!");
}
#[test]
/// starting with whitespace
fn starting_with_whitespace() {
    process_response_case("         hmmmmmmm...", "Whatever.");
}
#[test]
/// using acronyms in regular speech
fn using_acronyms_in_regular_speech() {
    process_response_case(
        "It's OK if you don't want to go work for NASA.",
        "Whatever.",
    );
}
#[test]
/// alternate silence
fn alternate_silence() {
    process_response_case("										", "Fine. Be that way!");
}
#[test]
/// prolonged silence
fn prolonged_silence() {
    process_response_case("          ", "Fine. Be that way!");
}

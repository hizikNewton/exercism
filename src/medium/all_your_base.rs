#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    todo!("Convert {number:?} from base {from_base} to base {to_base}")
}


Introduction
You've just been hired as professor of mathematics. Your first week went well, but something is off in your second week. The problem is that every answer given by your students is wrong! Luckily, your math skills have allowed you to identify the problem: the student answers are correct, but they're all in base 2 (binary)! Amazingly, it turns out that each week, the students use a different base. To help you quickly verify the student answers, you'll be building a tool to translate between bases.

Instructions
Convert a sequence of digits in one base, representing a number, into a sequence of digits in another base, representing the same number.

About Positional Notation
In positional notation, a number in base b can be understood as a linear combination of powers of b.

The number 42, in base 10, means:

(4 × 10¹) + (2 × 10⁰)

The number 101010, in base 2, means:

(1 × 2⁵) + (0 × 2⁴) + (1 × 2³) + (0 × 2²) + (1 × 2¹) + (0 × 2⁰)

The number 1120, in base 3, means:

(1 × 3³) + (1 × 3²) + (2 × 3¹) + (0 × 3⁰)

Yes. Those three numbers above are exactly the same. Congratulations!


use allyourbase as ayb;
#[test]
fn single_bit_one_to_decimal() {
    let input_base = 2;
    let input_digits = &[1];
    let output_base = 10;
    let output_digits = vec![1];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn binary_to_single_decimal() {
    let input_base = 2;
    let input_digits = &[1, 0, 1];
    let output_base = 10;
    let output_digits = vec![5];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn single_decimal_to_binary() {
    let input_base = 10;
    let input_digits = &[5];
    let output_base = 2;
    let output_digits = vec![1, 0, 1];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn binary_to_multiple_decimal() {
    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn decimal_to_binary() {
    let input_base = 10;
    let input_digits = &[4, 2];
    let output_base = 2;
    let output_digits = vec![1, 0, 1, 0, 1, 0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn trinary_to_hexadecimal() {
    let input_base = 3;
    let input_digits = &[1, 1, 2, 0];
    let output_base = 16;
    let output_digits = vec![2, 10];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn hexadecimal_to_trinary() {
    let input_base = 16;
    let input_digits = &[2, 10];
    let output_base = 3;
    let output_digits = vec![1, 1, 2, 0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn fifteen_bit_integer() {
    let input_base = 97;
    let input_digits = &[3, 46, 60];
    let output_base = 73;
    let output_digits = vec![6, 10, 45];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn empty_list() {
    let input_base = 2;
    let input_digits = &[];
    let output_base = 10;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn single_zero() {
    let input_base = 10;
    let input_digits = &[0];
    let output_base = 2;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn multiple_zeros() {
    let input_base = 10;
    let input_digits = &[0, 0, 0];
    let output_base = 2;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn leading_zeros() {
    let input_base = 7;
    let input_digits = &[0, 6, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
#[test]
#[ignore]
fn invalid_positive_digit() {
    let input_base = 2;
    let input_digits = &[1, 2, 1, 0, 1, 0];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidDigit(2))
    );
}
#[test]
#[ignore]
fn input_base_is_one() {
    let input_base = 1;
    let input_digits = &[];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidInputBase)
    );
}
#[test]
#[ignore]
fn output_base_is_one() {
    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 1;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidOutputBase)
    );
}
#[test]
#[ignore]
fn input_base_is_zero() {
    let input_base = 0;
    let input_digits = &[];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidInputBase)
    );
}
#[test]
#[ignore]
fn output_base_is_zero() {
    let input_base = 10;
    let input_digits = &[7];
    let output_base = 0;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidOutputBase)
    );
}

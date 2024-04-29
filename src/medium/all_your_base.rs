pub mod allyourbase {

    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        InvalidInputBase,
        InvalidOutputBase,
        InvalidDigit(u32),
    }
    pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
        if to_base < 2 {
            return Err(Error::InvalidOutputBase);
        }
        if from_base < 2 {
            return Err(Error::InvalidInputBase);
        }
        if let Some(x) = number.iter().find(|num| *num >= &from_base) {
            return Err(Error::InvalidDigit(*x));
        }
        let mut v = Vec::new();
        let mut base_10: Vec<u32> = number.to_vec();
        if number.iter().all(|num| *num == 0) {
            v.push(0);
            return Ok(v);
        }
        if from_base != 10 {
            base_10 = to_base_10(number.to_vec(), from_base)
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();
        }
        if to_base != 10 {
            return Ok(to_other_base_aside_10(&base_10, to_base));
        }
        return Ok(base_10);
    }

    pub fn to_base_10(number: Vec<u32>, to_base: u32) -> u32 {
        let to_base_10 = number
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc: u32, (idx, num)| {
                acc + num * (to_base.pow(idx as u32))
            });
        to_base_10
    }

    pub fn to_other_base_aside_10(number: &[u32], other_base: u32) -> Vec<u32> {
        let mut num: u32 = number
            .iter()
            .fold("".to_string(), |acc: String, e| acc + &e.to_string())
            .parse()
            .unwrap();
        let mut to_other_base = Vec::new();
        while num != 0 {
            to_other_base.push(num % other_base);
            num = num / other_base;
        }
        to_other_base.reverse();
        to_other_base
    }
}
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
fn output_base_is_zero() {
    let input_base = 10;
    let input_digits = &[7];
    let output_base = 0;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidOutputBase)
    );
}

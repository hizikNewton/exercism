// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        todo!("s, measured in seconds: {s}")
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}



use space_age::*;
fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}
#[test]
fn age_on_earth() {
    let seconds = 1000000000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    let expected = 31.69;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_mercury() {
    let seconds = 2134835688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_venus() {
    let seconds = 189839836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);
    let expected = 9.78;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_mars() {
    let seconds = 2129871239;
    let duration = Duration::from(seconds);
    let output = Mars::years_during(&duration);
    let expected = 35.88;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_jupiter() {
    let seconds = 901876382;
    let duration = Duration::from(seconds);
    let output = Jupiter::years_during(&duration);
    let expected = 2.41;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_saturn() {
    let seconds = 2000000000;
    let duration = Duration::from(seconds);
    let output = Saturn::years_during(&duration);
    let expected = 2.15;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_uranus() {
    let seconds = 1210123456;
    let duration = Duration::from(seconds);
    let output = Uranus::years_during(&duration);
    let expected = 0.46;
    assert_in_delta(expected, output);
}
#[test]
#[ignore]
fn age_on_neptune() {
    let seconds = 1821023456;
    let duration = Duration::from(seconds);
    let output = Neptune::years_during(&duration);
    let expected = 0.35;
    assert_in_delta(expected, output);
}

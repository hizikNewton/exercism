mod prime_factors;

use prime_factors::pfn;

fn main() {
    let prime_factors = pfn::factors(90125);
    println!("{:?}",prime_factors);
}

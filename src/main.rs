mod prime_factors;


fn main() {
    let prime_factors = prime_factors::factors(93819012551);
    println!("{:?}",prime_factors); 
}

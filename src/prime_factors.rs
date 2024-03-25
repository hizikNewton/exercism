pub mod pfn{
    pub fn factors(n: u64)  {
        let mut idx = 1;
        let mut start = 10;
        let prime  = gen_prime_factors(start);
        let mut res = n;
        let mut prime_factors = Vec::new();
        while res >1{
            for i in &prime{

                if res%i==0{
                    res/=i;
                    println!("{:?}",res);
                    prime_factors.push(i);
                    break;
                }
                continue;
            }
            idx+=1;
            prime = gen_prime_factors(start.pow(idx));
        }  
        println!("{:?}",prime_factors);
    
    }
    
    
    pub fn gen_prime_factors(n:u64)->Vec<u64>{
        let mut list:Vec<u64> = (2..n).collect();
        let mut p=2_u64;
        while p.pow(2)<=n{
            //set all multiples of 2 in the grid to it composite
            for i in p+1..n{
                if i%p==0 {
                    list[(i-2_u64) as usize]=0;
                }
            }
            p+=1;
        }

        let prime = list.into_iter().filter(|&x| x!=0).collect::<Vec<_>>();
        prime

    } 
}

#[test]
fn no_factors() {
    let factors = factors(1);
    let expected = [];
    assert_eq!(factors, expected);
}
#[test]
fn prime_number() {
    let factors = factors(2);
    let expected = [2];
    assert_eq!(factors, expected);
}
#[test]
fn another_prime_number() {
    let factors = factors(3);
    let expected = [3];
    assert_eq!(factors, expected);
}
#[test]
fn square_of_a_prime() {
    let factors = factors(9);
    let expected = [3, 3];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_first_prime() {
    let factors = factors(4);
    let expected = [2, 2];
    assert_eq!(factors, expected);
}
#[test]
fn cube_of_a_prime() {
    let factors = factors(8);
    let expected = [2, 2, 2];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_second_prime() {
    let factors = factors(27);
    let expected = [3, 3, 3];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_third_prime() {
    let factors = factors(625);
    let expected = [5, 5, 5, 5];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_first_and_second_prime() {
    let factors = factors(6);
    let expected = [2, 3];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_primes_and_non_primes() {
    let factors = factors(12);
    let expected = [2, 2, 3];
    assert_eq!(factors, expected);
}
#[test]
fn product_of_primes() {
    let factors = factors(901255);
    let expected = [5, 17, 23, 461];
    assert_eq!(factors, expected);
}
#[test]
fn factors_include_a_large_prime() {
    let factors = factors(93819012551);
    let expected = [11, 9539, 894119];
    assert_eq!(factors, expected);
}

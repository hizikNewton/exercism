/* mod prime_factors; 
mod bob;
use bob::bob_risky;
mod highscore;*/

mod kg_garden;

use kg_garden::kindergarten_garden::plants;
fn main() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    println!("whattt {:?}",plants(diagram, student));
   /*  let prime_factors = prime_factors::factors(93819012551);
    println!("{:?}",prime_factors);  
    println!("{:?}",bob_risky::reply("1, 2, 3 GO!"));*/
}

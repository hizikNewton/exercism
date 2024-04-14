/* Instructions
Given a diagram, determine which plants each child in the kindergarten class is responsible for.

The kindergarten class is learning about growing plants. The teacher thought it would be a good idea to give them actual seeds, plant them in actual dirt, and grow actual plants.

They've chosen to grow grass, clover, radishes, and violets.

To this end, the children have put little cups along the window sills, and planted one type of plant in each cup, choosing randomly from the available types of seeds.

[window][window][window]
........................ # each dot represents a cup
........................
There are 12 children in the class:

Alice, Bob, Charlie, David,
Eve, Fred, Ginny, Harriet,
Ileana, Joseph, Kincaid, and Larry.
Each child gets 4 cups, two on each row. Their teacher assigns cups to the children alphabetically by their names.

The following diagram represents Alice's plants:

[window][window][window]
VR......................
RG......................
In the first row, nearest the windows, she has a violet and a radish. In the second row she has a radish and some grass.

Your program will be given the plants from left-to-right starting with the row nearest the windows. From this, it should be able to determine which plants belong to each student.

For example, if it's told that the garden looks like so:

[window][window][window]
VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV
Then if asked for Alice's plants, it should provide:

Violets, radishes, violets, radishes
While asking for Bob's plants would yield:

Clover, grass, clover, clover
 */
pub mod kindergarten_garden{
use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {

    let mut plant_code = HashMap::new();
    let pairs:Vec<(char, &'static str)> = vec![
        ('C',"clover"),
        ('V',"violets"),
        ('R',"radishes"),
        ('G',"grass")
        ];
    for &(key, value) in &pairs {
            plant_code.insert(key, value);
        }
    let mut all_pupil_plant:HashMap<&str, String> = HashMap::new();
    let rows:Vec<&str> = diagram.split('\n').collect();
    for row in &rows{
        let mut pupil = [ "Alice", "Bob", "Charlie", "David","Eve", "Fred", "Ginny", "Harriet","Ileana", "Joseph", "Kincaid",  "Larry"].iter();
        let nrow:Vec<char> = row.chars().collect();
        for (i,j) in nrow.iter().enumerate().step_by(2){   
            let current_plant = format!("{}{}",j,nrow[i+1]);
            if let Some(pupil) =  pupil.next() {
                let former_plant = all_pupil_plant.get(pupil);
                match former_plant {
                   Some(fp)=> {
                        all_pupil_plant.insert(pupil,format!("{}{}",fp,current_plant));

                    },
                   None=> {
                    all_pupil_plant.insert(pupil,current_plant);}
                };
            };
            
     }
    }
    let pupil_plant = all_pupil_plant.get(student);
    
    let mut result:Vec<&'static str> = Vec::new();
    if let Some(p) = pupil_plant {
        
             p.chars().for_each(|c| {
                 if let Some(code) = plant_code.get(&c){
                    result.push(*code);
                } 
            });
        
    };
    result

}
}





 use kindergarten_garden::*;
#[test]
fn garden_with_single_student() {
    let diagram = "RC
GG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn different_garden_with_single_student() {
    let diagram = "VC
RC";
    let student = "Alice";
    let expected = vec!["violets", "clover", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn garden_with_two_students() {
    let diagram = "VVCG
VVRC";
    let student = "Bob";
    let expected = vec!["clover", "grass", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn second_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Bob";
    let expected = vec!["clover", "clover", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn third_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Charlie";
    let expected = vec!["grass", "grass", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_alice_first_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let expected = vec!["violets", "radishes", "violets", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_bob_second_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Bob";
    let expected = vec!["clover", "grass", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_charlie() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Charlie";
    let expected = vec!["violets", "violets", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_david() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "David";
    let expected = vec!["radishes", "violets", "clover", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_eve() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Eve";
    let expected = vec!["clover", "grass", "radishes", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_fred() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Fred";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_ginny() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ginny";
    let expected = vec!["clover", "grass", "grass", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_harriet() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Harriet";
    let expected = vec!["violets", "radishes", "radishes", "violets"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_ileana() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ileana";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_joseph() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Joseph";
    let expected = vec!["violets", "clover", "violets", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_kincaid_second_to_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Kincaid";
    let expected = vec!["grass", "clover", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]
fn for_larry_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Larry";
    let expected = vec!["grass", "violets", "clover", "violets"];
    assert_eq!(plants(diagram, student), expected);
}

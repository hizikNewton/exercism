
pub mod frequency{
    use std::collections::{ HashMap};
    use std::sync::Arc;
    use std::{thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result:HashMap<char, usize> = HashMap::new();

    if input.is_empty(){
        return result;
    }

    let worklists = create_chunk(input, worker_count);

    let mut thread_handles = vec![];

    for worklist in worklists{
            thread_handles.push(thread::spawn(move || process_files(worklist)));
        }
    
    for handle in thread_handles {
        let res = handle.join().unwrap();
        res.keys().for_each(|k| {
            match result.get(k) {
                Some(count)=>result.insert(*k, res.get(k).unwrap()+count),
                None=>result.insert(*k, *res.get(k).unwrap())
            };
        })
    }

    result

}

fn create_chunk(input: &[&str], worker_count: usize) -> Vec<Vec<String>> {
    // Clone the input slice into owned data to avoid lifetime issues
    let input_owned: Vec<String> = input.iter().map(|&s| s.to_string()).collect();
    if input_owned.len()>worker_count{
    let chunk_size = input_owned.len() / worker_count;
    input_owned
        .chunks(chunk_size) // Split the input into chunks
        .map(|chunk| chunk.to_vec()) // Convert each chunk into a vector
        .collect() }
    }// Collect the vectors into a vector of vectors
}

pub fn process_files(input: Vec<String>)->HashMap<char, usize>{
   
    let mut hash_map:HashMap<char, usize> = HashMap::new();
     input.iter().for_each(|str| str.to_lowercase().chars().for_each( |ch| {
        match hash_map.get(&ch) {
            Some(count)=>hash_map.insert(ch, count+1),
            None=>hash_map.insert(ch, 1_usize)
        };
    }
    ));
    hash_map
}

pub fn split_vec_into_chunks<'a>(input: Arc<& 'a[&str]>, worker_count: usize)-> Vec<&'a[&'a str]>{
    if input.is_empty(){
       return Vec::new();
    }
    let result = input.chunks(worker_count).collect::<Vec<_>>();
    result
}

}

use std::collections::HashMap;

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];
// Dutch national anthem
const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];
// American national anthem
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];
#[test]
fn no_texts() {
    assert_eq!(frequency::frequency(&[], 4), HashMap::new());
}
#[test]
fn one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(frequency::frequency(&["a"], 4), hm);
}
#[test]
fn case_insensitivity() {
    let mut hm = HashMap::new();
    hm.insert('a', 2);
    assert_eq!(frequency::frequency(&["aA"], 4), hm);
}
#[test]
fn many_empty_lines() {
    let v = vec![""; 1000];
    assert_eq!(frequency::frequency(&v[..], 4), HashMap::new());
}
#[test]
fn many_times_same_text() {
    let v = vec!["abc"; 1000];
    let mut hm = HashMap::new();
    hm.insert('a', 1000);
    hm.insert('b', 1000);
    hm.insert('c', 1000);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}
#[test]
fn punctuation_doesnt_count() {
    assert!(!frequency::frequency(&WILHELMUS, 4).contains_key(&','));
}
#[test]
fn numbers_dont_count() {
    assert!(!frequency::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
}
#[test]
fn all_three_anthems_1_worker() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}
#[test]
fn all_three_anthems_3_workers() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}
#[test]
fn non_integer_multiple_of_threads() {
    let v = vec!["abc"; 999];
    let mut hm = HashMap::new();
    hm.insert('a', 999);
    hm.insert('b', 999);
    hm.insert('c', 999);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}

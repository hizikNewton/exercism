pub mod high_scores{
    
#[derive(Debug)]
pub struct HighScores<'a>{
    scores:  &'a[u32]
}

impl<'a> HighScores<'a>{
    pub fn new(scores:&'a [u32]) -> Self {
        HighScores{scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let len = self.scores.len();
        if len>0 {

        Some(self.scores[self.scores.len()-1])
        }else{
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let len = self.scores.len();
        if len>0 {
            let mut best = self.scores[0];
            for (i,j) in self.scores.iter().enumerate(){
                if i<(len-1) && (self.scores[i] > self.scores[i+1]) {
                    best = *j;
                } 
            }
            Some(best)
         }else{
            None
         }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let len = self.scores.len();
        let mut scores:Vec<u32> = Vec::new();
        for i in self.scores{
            scores.push((*i).try_into().unwrap())
        }
        scores.sort();
        scores.reverse();
        if len>=3 {  
            scores.as_slice()[0..3].to_vec()
        }else if len > 0 && len<3 {
            scores
        }else{
            Vec::new()
        }
    }
}
}

use high_scores::HighScores;
#[test]
fn list_of_scores() {
    let expected = [30, 50, 20, 70];
    let high_scores = HighScores::new(&expected);
    assert_eq!(high_scores.scores(), &expected);
}
#[test]
fn latest_score() {
    let high_scores = HighScores::new(&[100, 0, 90, 30]);
    assert_eq!(high_scores.latest(), Some(30));
}
#[test]
fn latest_score_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.latest(), None);
}
#[test]
fn personal_best() {
    let high_scores = HighScores::new(&[40, 100, 70]);
    assert_eq!(high_scores.personal_best(), Some(100));
}
#[test]
fn personal_best_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.personal_best(), None);
}
#[test]
fn personal_top_three() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
    assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
}
#[test]
fn personal_top_three_highest_to_lowest() {
    let high_scores = HighScores::new(&[20, 10, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
}
#[test]
fn personal_top_three_with_tie() {
    let high_scores = HighScores::new(&[40, 20, 40, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
}
#[test]
fn personal_top_three_with_less_than_three_scores() {
    let high_scores = HighScores::new(&[30, 70]);
    assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
}
#[test]
fn personal_top_three_only_one_score() {
    let high_scores = HighScores::new(&[40]);
    assert_eq!(high_scores.personal_top_three(), vec![40]);
}
#[test]
fn personal_top_three_empty() {
    let high_scores = HighScores::new(&[]);
    assert!(high_scores.personal_top_three().is_empty());
}

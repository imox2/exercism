#[derive(Debug)]
pub struct HighScores {
    score_list: Vec<u32>,
    score_list_sorted: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut scores_vec = Vec::new();
        for i in scores.iter() {
            scores_vec.push(*i);
        }
        let mut scores_vec_copy = scores_vec.to_vec();
        scores_vec_copy.sort();
        scores_vec_copy.reverse();
        HighScores {
            score_list: scores_vec,
            score_list_sorted: scores_vec_copy, 
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.score_list[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match &self.score_list.last() {
            Some(last) => Some(**last),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut max = 0;
        for i in &self.score_list {
            if i > &max {
                max = *i;
            }
        }
        if max == 0 {
            return None 
        } else { 
            return Some(max) 
        };
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_vec = Vec::new();
        let mut count = 0;
        for i in &self.score_list_sorted {
            if count == 3 {
                break;
            } else {
                scores_vec.push(*i);
                count += 1;
            }
        }
        scores_vec
    }
}

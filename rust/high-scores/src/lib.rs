#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(last) => Some(*last),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
			Some(score) => Some(*score),
			None => None,
		}
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_vec = self.scores().to_vec();
        scores_vec.sort_unstable();
        scores_vec.reverse();
        scores_vec.truncate(3);
        scores_vec
    }
}
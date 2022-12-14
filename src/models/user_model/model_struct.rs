use super::avg;

struct Model {
    model: Vec<Vec<[i32; 9]>>,
}

impl Model {
    fn empty() -> Self {
        Self {
            model: vec![
                vec![[0; 9]; 5],
                vec![[0; 9]; 6],
                vec![[0; 9]; 5],
                vec![[0; 9]; 20],
                vec![[0; 9]; 49],
                vec![[0; 9]; 5],
            ],
        }
    }

    fn affinity() -> Self {
        Self {
            model: vec![
                vec![[1001; 9]; 5],
                vec![[1001; 9]; 6],
                vec![[1001; 9]; 5],
                vec![[1001; 9]; 20],
                vec![[1001; 9]; 49],
                vec![[1001; 9]; 5],
            ],
        }
    }

    fn average() -> Self {
        Self {
            model: avg::model()
        }
    }

    // general stats

    fn general(&self, index: usize) -> &[i32; 9] {
        &self.model[0][index]
    }

    fn list_length(&self) -> &i32 {
        &self.model[0][0][0]
    }

    fn avg_score(&self) -> &i32 {
        &self.model[0][0][1]
    }

    fn avg_score_deviation(&self) -> &i32 {
        &self.model[0][0][2]
    }

    fn avg_scored_percentage(&self) -> &i32 {
        &self.model[0][0][3]
    }

    // detailed stats

    fn airing_decades(&self, index: usize) -> &[i32; 9] {
        &self.model[1][index]
    }

    fn ratings(&self, index: usize) -> &[i32; 9] {
        &self.model[2][index]
    }

    fn series_length(&self, index: usize) -> &[i32; 9] {
        &self.model[3][index]
    }

    fn gernes(&self, index: usize) -> &[i32; 9] {
        &self.model[4][index]
    }

    fn themes(&self, index: usize) -> &[i32; 9] {
        &self.model[5][index]
    }

    fn demographics(&self, index: usize) -> &[i32; 9] {
        &self.model[6][index]
    }
}

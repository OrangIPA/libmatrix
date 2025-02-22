#[derive(Debug)]
pub struct Matrix {
    items: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        let zero_vec = vec![0 as f64; height];
        let mut ret: Vec<Vec<f64>> = vec![];

        for _ in 0..width {
            ret.push(zero_vec.clone());
        }

        Self { items: ret }
    }

    pub fn from_vec(val: Vec<Vec<f64>>) -> Option<Self> {
        let length = val.get(0)?.len();
        let same = val.iter().all(|v| v.len() == length);
        if !same {
            return None;
        }

        Some(Self { items: val })
    }

    pub fn size(&self) -> (usize, usize) {
        (
            self.items.get(0).unwrap_or(&(vec![] as Vec<f64>)).len(),
            self.items.len(),
        )
    }

    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.items.get(i)?.get(j).cloned()
    }

    pub fn set(&mut self, i: usize, j: usize, val: f64) -> Option<()> {
        let i = self.items.get_mut(i)?.get_mut(j)?;
        *i = val;

        Some(())
    }

    pub fn add_with(&self, other: &Self) -> Option<Matrix> {
        if self.size() != other.size() {
            return None;
        }

        Some(Matrix {
            items: self
                .items
                .iter()
                .zip(other.items.iter())
                .map(|v| v.0.iter().zip(v.1.iter()).map(|v| v.0 + v.1).collect())
                .collect(),
        })
    }
}

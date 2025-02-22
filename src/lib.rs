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

    /// Only work with vector (n by 1 matrix)
    pub fn dot_with(&self, other: &Self) -> Option<f64> {
        if self.size() != other.size() || self.size().1 != 1 {
            return None;
        }

        let this = self.items.get(0).unwrap();
        let that = other.items.get(0).unwrap();

        this.iter()
            .zip(that.iter())
            .map(|v| v.0 * v.1)
            .reduce(|acc, v| acc + v)
    }

    /// Only work with 3 by 1 matrix
    pub fn cross_with(&self, other: &Matrix) -> Option<Self> {
        if self.size() != (3, 1) || other.size() != (3, 1) {
            return None;
        }

        let this = self.items.get(0).unwrap();
        let that = other.items.get(0).unwrap();

        let res: Vec<f64> = vec![
            this[1] * that[2] - this[2] * that[1],
            this[2] * that[0] - this[0] * that[2],
            this[0] * that[1] - this[1] * that[0],
        ];

        Some(Self { items: vec![res] })
    }
}

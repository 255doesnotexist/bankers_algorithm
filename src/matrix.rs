use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<i32>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn from_vec(data: Vec<Vec<i32>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, "\t")?;
                }
                write!(f, "{:>4}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
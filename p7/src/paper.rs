use crate::matrix::{Matrix, MatrixDimensions};
use crate::sheet::Sheet;

use std::fmt;

pub struct Paper {
    matrix: Vec<Vec<char>>,
}

impl Paper {
    pub fn new(dimesions: MatrixDimensions) -> Self {
        let area = dimesions.area();

        if area % 2 != 0 {
            panic!(format!("Bad paper: area should be even, got {}", area));
        }

        let MatrixDimensions { height, width } = dimesions;

        Self {
            matrix: vec![vec![' '; width]; height],
        }
    }

    pub fn draw(&mut self, sheet: &Sheet, chars: &[char]) {
        let MatrixDimensions { height, width } = self.dimesions();

        let mut current_char = 0;

        for row in 0..height {
            for col in 0..width {
                if sheet.hole_at(row, col) {
                    self.matrix[row][col] = chars[current_char];
                    current_char += 1;
                }
            }
        }
    }

    pub fn read(&self, sheet: &Sheet) -> Vec<char> {
        let MatrixDimensions { height, width } = self.dimesions();

        let mut chars = vec![];

        for row in 0..height {
            for col in 0..width {
                if sheet.hole_at(row, col) {
                    chars.push(self.matrix[row][col]);
                }
            }
        }

        chars
    }
}

impl Matrix<char> for Paper {
    fn get_matrix(&self) -> &Vec<Vec<char>> {
        &self.matrix
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MatrixDimensions { height, width } = self.dimesions();

        for row in 0..height {
            write!(f, "\n[")?;
            for col in 0..width {
                write!(f, "{}", self.matrix[row][col])?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
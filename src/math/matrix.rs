use std::{fmt::Debug, ops::Mul};

#[derive(Debug)]
pub struct Matrix<T>
where
  T:  Mul<T>,
{
  _matrix: Vec<Vec<T>>,
  num_rows: usize,
  num_columns: usize,
}

impl<T> Matrix<T>
where
  T: Mul<T>,
{
  fn is_valid_matrix(matrix: &Vec<Vec<T>>) -> bool {
    for vec in matrix {
      if vec.len() != matrix[0].len() {
        return false;
      }
    }
    return true;
  }

  pub fn new(matrix: Vec<Vec<T>>) -> Matrix<T> {
    if !Matrix::is_valid_matrix(&matrix) {
      panic!("Not a valid matrix");
    }

    let num_rows = matrix.len();
    let num_columns = matrix[0].len();

    return Matrix {
      _matrix: matrix,
      num_rows,
      num_columns,
    };
  }
}

impl<TScalar, T> Mul<TScalar> for Matrix<T>
where
  TScalar: Mul<T> + Into<T> + Copy,
  T: Mul<T> + Mul<TScalar, Output = T>
{
  type Output = Matrix<T>;
  fn mul(self, rhs: TScalar) -> Self::Output {
    let mut new_matrix = Vec::<Vec<T>>::new();

    for vec in self._matrix {
      let mut new_vec: Vec<T> = Vec::new();
      for scalar in vec {
        new_vec.push(scalar * rhs);
      }
      new_matrix.push(new_vec);
    }

    Matrix::new(new_matrix)
  }
}

impl<T> PartialEq for Matrix<T> where T: PartialEq<T> + Mul<T> {
  fn eq(&self, other: &Self) -> bool {
    if self.num_columns != other.num_columns {
      return false;
    }
    if self.num_rows != other.num_rows {
      return false;
    }
    for vec_i in 0..self._matrix.len() {
      for scalar_i in 0..self._matrix[vec_i].len() {
        if self._matrix[vec_i][scalar_i] != other._matrix[vec_i][scalar_i] {
          return false;
        }
      }
    }
    return true;
  }
  fn ne(&self, other: &Self) -> bool {
      !self.eq(other)
  }
}

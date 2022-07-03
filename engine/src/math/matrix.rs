use crate::math::trig;
use std::ops::{Add, Index, Mul, Sub};

pub trait Determinant<T> {
  fn det(&self) -> T;
}

pub trait IdentityMatrix<T> {
  /// Returns the identity for a given matrix
  fn identity(&self) -> Matrix<T>;
  /// Returns the identity for a given type using num rows
  /// Eg, for i32 that would be 1
  fn generate_identity(num_rows: usize) -> Matrix<T>;
}

pub trait RotationMatrix<T> {
  fn rotate_z(&self, deg: T) -> Matrix<T>;
}

pub trait ScaleMatrix<T> {
  fn generate_scale_matrix(x: T, y: T, z: T) -> Matrix<T>;
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
  matrix: Vec<Vec<T>>,
  flattened: Vec<T>,
  num_rows: usize,
  num_columns: usize,
}

impl<T> Matrix<T> {
  fn is_valid_matrix(matrix: &Vec<Vec<T>>) -> bool {
    for vec in matrix {
      if vec.len() != matrix[0].len() {
        return false;
      }
    }
    return true;
  }

  fn flatted_matrix_vec(matrix_vec: &Vec<Vec<T>>) -> Vec<T>
  where
    T: Clone,
  {
    let mut flattened = Vec::<T>::new();

    for vec in matrix_vec {
      for val in vec {
        flattened.push(val.to_owned());
      }
    }

    flattened
  }

  pub fn get_num_columns(&self) -> usize {
    self.num_columns
  }

  pub fn get_num_rows(&self) -> usize {
    self.num_rows
  }

  pub fn len(&self) -> usize {
    self.get_num_rows()
  }

  pub fn get_inner_ptr(&self) -> &Vec<T> {
    &self.flattened
  }

  pub fn new(matrix: Vec<Vec<T>>) -> Matrix<T>
  where
    T: Clone,
  {
    if !Matrix::is_valid_matrix(&matrix) {
      panic!("Not a valid matrix");
    }

    let num_rows = matrix.len();
    let num_columns = matrix[0].len();

    let flattened = Matrix::flatted_matrix_vec(&matrix);

    return Matrix {
      matrix,
      num_rows,
      num_columns,
      flattened,
    };
  }
}

impl<T> Index<usize> for Matrix<T> {
  type Output = Vec<T>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.matrix[index]
  }
}

impl<T, TOther> Mul<Matrix<TOther>> for Matrix<T>
where
  TOther: Mul<T> + Into<T> + Copy,
  T: Mul<T>
    + Mul<TOther, Output = T>
    + Sub<T, Output = T>
    + Add<T, Output = T>
    + Copy,
{
  type Output = Matrix<T>;
  fn mul(self, rhs: Matrix<TOther>) -> Self::Output {
    if self.get_num_columns() != rhs.get_num_rows() {
      panic!("Not a valid matrix multiplication")
    }

    let mut new_matrix = Vec::<Vec<T>>::new();
    new_matrix.reserve(self.get_num_rows());

    for row_i in 0..self.get_num_rows() {
      let mut row = Vec::<T>::new();

      row.reserve(rhs.get_num_columns());
      for col_i in 0..rhs.get_num_columns() {
        let mut idx_sum = self[0][0] - self[0][0];
        for rhs_row_i in 0..rhs.get_num_rows() {
          idx_sum = idx_sum + (self[row_i][rhs_row_i] * rhs[rhs_row_i][col_i])
        }
        row.push(idx_sum)
      }
      new_matrix.push(row);
    }

    Matrix::new(new_matrix)
  }
}

impl<T, TOther> Add<Matrix<TOther>> for Matrix<T>
where
  TOther: Add<T> + Into<T> + Copy,
  T: Add<T> + Add<TOther, Output = T> + Copy,
{
  type Output = Matrix<T>;
  fn add(self, rhs: Matrix<TOther>) -> Self::Output {
    let mut new_matrix = Vec::<Vec<T>>::new();

    if (self.get_num_columns() != rhs.get_num_columns())
      || (self.get_num_rows() != rhs.get_num_rows())
    {
      panic!("Matrices don't have same number of columns or rows");
    }

    let num_columns = self.get_num_columns();

    for vec_i in 0..self.len() {
      let mut new_vec = Vec::<T>::new();
      new_vec.reserve(num_columns);
      for scalar_i in 0..self[vec_i].len() {
        new_vec.push(self[vec_i][scalar_i] + rhs[vec_i][scalar_i]);
      }
      new_matrix.push(new_vec);
    }

    Matrix::new(new_matrix)
  }
}

impl<TScalar, T> Mul<TScalar> for Matrix<T>
where
  TScalar: Mul<T> + Into<T> + Copy,
  T: Mul<T> + Mul<TScalar, Output = T> + Clone,
{
  type Output = Matrix<T>;
  fn mul(self, rhs: TScalar) -> Self::Output {
    let mut new_matrix = Vec::<Vec<T>>::new();

    for vec in self.matrix {
      let mut new_vec: Vec<T> = Vec::new();
      for scalar in vec {
        new_vec.push(scalar * rhs);
      }
      new_matrix.push(new_vec);
    }

    Matrix::new(new_matrix)
  }
}

impl<T> PartialEq for Matrix<T>
where
  T: PartialEq<T> + Mul<T>,
{
  fn eq(&self, other: &Self) -> bool {
    if self.num_columns != other.num_columns {
      return false;
    }
    if self.num_rows != other.num_rows {
      return false;
    }
    for vec_i in 0..self.len() {
      for scalar_i in 0..self[vec_i].len() {
        if self[vec_i][scalar_i] != other[vec_i][scalar_i] {
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

impl<T> Determinant<T> for Matrix<T>
where
  T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
  /// Calculates determinant using Laplace expansion
  fn det(&self) -> T {
    if self.num_columns != self.num_rows {
      panic!("Matrix is not a square");
    }

    if self.num_columns == 2 && self.num_rows == 2 {
      return (self[0][0] * self[1][1]) - (self[1][0] * self[0][1]);
    }

    let mut determinant = self[0][0] - self[0][0];

    for col_i in 0..self.get_num_columns() {
      let scalar = self[0][col_i];

      let mut smaller_mat = Vec::<Vec<T>>::new();

      // Skipping the row that includes the scalar
      for row_i in 1..self.get_num_rows() {
        let mut new_row = Vec::<T>::new();

        for item_i in 0..self[row_i].len() {
          if item_i == col_i {
            continue;
          }
          new_row.push(self[row_i][item_i]);
        }

        smaller_mat.push(new_row);
      }

      let smaller_mat_mat = Matrix::new(smaller_mat);

      let block_determinant = smaller_mat_mat.det() * scalar;

      if col_i == 0 || col_i % 2 == 0 {
        determinant = determinant + block_determinant;
      } else {
        determinant = determinant - block_determinant;
      }
    }
    determinant
  }
}

impl IdentityMatrix<f32> for Matrix<f32> {
  fn identity(&self) -> Matrix<f32> {
    Matrix::generate_identity(self.get_num_rows())
  }

  fn generate_identity(num_rows: usize) -> Matrix<f32> {
    let mut new_matrix = Vec::<Vec<f32>>::new();
    new_matrix.reserve(num_rows);
    for i in 0..num_rows {
      let mut new_row = Vec::<f32>::new();
      for j in 0..num_rows {
        if i == j {
          new_row.push(1.0);
        } else {
          new_row.push(0.0);
        }
      }
      new_matrix.push(new_row);
    }
    Matrix::new(new_matrix)
  }
}

impl IdentityMatrix<i32> for Matrix<i32> {
  fn identity(&self) -> Matrix<i32> {
    Matrix::generate_identity(self.get_num_rows())
  }
  fn generate_identity(num_rows: usize) -> Matrix<i32> {
    let mut new_matrix = Vec::<Vec<i32>>::new();
    new_matrix.reserve(num_rows);
    for i in 0..num_rows {
      let mut new_row = Vec::<i32>::new();
      for j in 0..num_rows {
        if i == j {
          new_row.push(1);
        } else {
          new_row.push(0);
        }
      }
      new_matrix.push(new_row);
    }
    Matrix::new(new_matrix)
  }
}

impl RotationMatrix<f32> for Matrix<f32> {
  fn rotate_z(&self, deg: f32) -> Matrix<f32> {
    let rotation_z_matrix = Matrix::<f32>::new(vec![
      vec![trig::get_cos(deg), -trig::get_sin(deg), 0.0, 0.0],
      vec![trig::get_sin(deg), trig::get_cos(deg), 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0],
    ]);

    self.to_owned() * rotation_z_matrix
  }
}

impl ScaleMatrix<f32> for Matrix<f32> {
  fn generate_scale_matrix(x: f32, y: f32, z: f32) -> Matrix<f32> {
    Matrix::new(vec![
      vec![x, 0.0, 0.0, 0.0],
      vec![0.0, y, 0.0, 0.0],
      vec![0.0, 0.0, z, 0.0],
      vec![0.0, 0.0, 0.0, 1.0],
    ])
  }
}

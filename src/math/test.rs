#[cfg(test)]
mod tests {
  #[test]
  fn scalar_matrix_multiply_ok() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let test_matrix = Matrix::new(vec![vec![2, 4, 6], vec![8, 10, 12]]);
    assert_eq!(matrix * 2, test_matrix);
  }

  #[test]
  fn scalar_matrix_multiply_bad() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let test_matrix = Matrix::new(vec![vec![2, 4, 6], vec![8, 10, 12]]);
    assert_ne!(matrix * 3, test_matrix);
  }

  #[test]
  #[should_panic(expected = "Not a valid matrix")]
  fn matrix_creation_validates() {
    use crate::math::matrix::Matrix;
    // Unequal columns
    Matrix::new(vec![vec![1, 2], vec![1, 2, 3]]);
  }

  #[test]
  fn matrix_matrix_add_ok() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![1, -7, 5], vec![0, 3, -10]]);
    let rhs_matrix = Matrix::new(vec![vec![5, 0, 3], vec![11, -1, 7]]);
    let test_matrix = Matrix::new(vec![vec![6, -7, 8], vec![11, 2, -3]]);
    assert_eq!(matrix + rhs_matrix, test_matrix);
  }

  #[test]
  fn matrix_matrix_add_bad() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![1, -7, 5], vec![0, 3, -10]]);
    let rhs_matrix = Matrix::new(vec![vec![5, 0, 3], vec![11, -1, 7]]);
    let test_matrix = Matrix::new(vec![vec![6, 0, 0], vec![11, 2, -3]]);
    assert_ne!(matrix + rhs_matrix, test_matrix);
  }

  #[test]
  #[should_panic(
    expected = "Matrices don't have same number of columns or rows"
  )]
  fn matrix_matrix_add_validation() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![1, -7, 5], vec![0, 3, -10]]);
    let rhs_matrix = Matrix::new(vec![vec![5, 0], vec![11, -1]]);
    let _ = matrix + rhs_matrix;
  }

  #[test]
  fn matrix_2x2_determinant() {
    use crate::math::matrix::{Determinant, Matrix};
    let matrix = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let test_det = -2;
    assert_eq!(matrix.det(), test_det);
  }

  #[test]
  fn matrix_3x3_determinant() {
    use crate::math::matrix::{Determinant, Matrix};
    let matrix = Matrix::new(vec![vec![6, 1, 1], vec![4, -2, 5], vec![2, 8, 7]]);
    let test_det = -306;
    assert_eq!(matrix.det(), test_det);
  }

  #[test]
  fn matrix_4x4_determinant() {
    use crate::math::matrix::{Determinant, Matrix};
    let matrix = Matrix::new(vec![
      vec![6, 1, 1, 3],
      vec![4, -2, 5, 10],
      vec![2, 8, 7, -8],
      vec![7, 9, 10, -9],
    ]);
    let test_det = 1464;
    assert_eq!(matrix.det(), test_det);
  }

  #[test]
  fn matrix_5x5_determinant() {
    use crate::math::matrix::{Determinant, Matrix};
    let matrix = Matrix::new(vec![
      vec![5, 6, 19, 10, 0],
      vec![12, 17, 8, 18, 9],
      vec![10, 10, 22, 2, 3],
      vec![67, 8, 9, 0, 0],
      vec![-3, 45, 3, 2, 1],
    ]);
    let test_det = 6709072;
    assert_eq!(matrix.det(), test_det);
  }

  #[test]
  #[should_panic(expected = "Matrix is not a square")]
  fn matrix_determinant_validates() {
    use crate::math::matrix::{Determinant, Matrix};
    let matrix = Matrix::new(vec![
      vec![6, 1, 1],
      vec![4, -2, 5],
      vec![2, 8, 7],
      vec![7, 9, 10],
    ]);
    let test_det = 1464;
    assert_eq!(matrix.det(), test_det);
  }

  #[test]
  fn matrix_2x3_dot_3x2_ok() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![0, 3, 5], vec![5, 5, 2]]);
    let rhs = Matrix::new(vec![vec![3, 4], vec![3, -2], vec![4, -2]]);
    let test_mat = Matrix::new(vec![vec![29, -16], vec![38, 6]]);
    assert_eq!(matrix * rhs, test_mat);
  }

  #[test]
  fn matrix_3x3_dot_3x3_ok() {
    use crate::math::matrix::Matrix;
    let matrix =
      Matrix::new(vec![vec![12, 7, 8], vec![99, -1, 8], vec![7, 6, -1]]);
    let rhs = Matrix::new(vec![vec![28, 9, 0], vec![0, 12, 3], vec![-9, 8, 10]]);
    let test_mat = Matrix::new(vec![
      vec![264, 256, 101],
      vec![2700, 943, 77],
      vec![205, 127, 8],
    ]);
    assert_eq!(matrix * rhs, test_mat);
  }

  fn matrix_2x1_dot_1x2_ok() {
    use crate::math::matrix::Matrix;
    let matrix =
      Matrix::new(vec![vec![12, 7, 8], vec![99, -1, 8], vec![7, 6, -1]]);
    let rhs = Matrix::new(vec![vec![28, 9, 0], vec![0, 12, 3], vec![-9, 8, 10]]);
    let test_mat = Matrix::new(vec![
      vec![264, 256, 101],
      vec![2700, 943, 77],
      vec![205, 127, 8],
    ]);
    assert_eq!(matrix * rhs, test_mat);
  }

  #[test]
  #[should_panic(expected = "Not a valid matrix multiplication")]
  fn matrix_dot_validates() {
    use crate::math::matrix::Matrix;
    let matrix = Matrix::new(vec![vec![0, 3, 5], vec![5, 5, 2]]);
    let rhs = Matrix::new(vec![vec![3, 4], vec![3, -2]]);
    let _ = matrix * rhs;
  }

  #[test]
  fn matrix_identity() {
    use crate::math::matrix::{Matrix, IdentityMatrix};
    let matrix = Matrix::new(vec![vec![0, 3, 5], vec![5, 5, 2], vec![4,4,4]]);
    let test_mat = Matrix::new(vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]]);
    assert_eq!(matrix.identity(), test_mat);
  }
}

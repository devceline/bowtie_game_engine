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
}

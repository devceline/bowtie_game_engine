pub fn absolute_value_f32(val: f32) -> f32 {
  if val < 0.0 {
    val * -1.0
  } else {
    val
  }
}

/*
 * Normalizes num to be between min and max
 */
pub fn normalize<T>(num: T, min: T, max: T) -> f32
where
  T: Into<f32> + Copy,
{
  (max.into() - min.into())
    * ((num.into() - min.into()) / (max.into() - min.into()))
    + min.into()
}

use std::collections::HashMap;

use crate::StandardEntity;

#[derive(Clone, Debug)]
pub enum Value {
  Null,
  Bool(bool),
  Number(f32),
  String(String),
  Array(Vec<Value>),
  Object(HashMap<String, Value>),
  Vec2f32((f32, f32)),
}

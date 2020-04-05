use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
  pub values: Vec<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}


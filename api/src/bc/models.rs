use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub a: i32,
    pub a_plus_b: i32,
    pub a_plus_c: i32
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub b: i32,
    pub c: i32
}
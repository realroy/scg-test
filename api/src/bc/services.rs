use crate::bc::{Input, Output};

pub fn find_bc_service(input: Input) -> Output {
  return Output {
    b: input.a_plus_b - input.a,
    c: input.a_plus_c - input.a,
  };
}
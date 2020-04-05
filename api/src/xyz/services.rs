use crate::xyz::Output;

pub fn find_xyz_service(v: Vec<i32>) -> Output {
  let l = v.len() as i32;
  let gap: i32 = (v[1] - v[0]) / 2;
  
  let first = v[0];
  let last_index = (l - 1) as usize;
  let last = v[last_index];
  
    
  let x = first + (gap * -2);
  let y = first + (gap * -1);
  let z = last + (gap * (l - 1 + 2));

  return Output { x: x, y:y, z:z };
}
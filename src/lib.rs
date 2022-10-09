#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn one() -> String {
  xid::new().to_string()
}

#[napi]
pub fn many(size: u32) -> Vec<String> {
  let mut result = Vec::with_capacity(size as usize);
  for _ in 0..size {
    result.push(xid::new().to_string());
  }

  result
}

#![deny(clippy::all)]

use napi::{bindgen_prelude::Array, Env};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn one() -> String {
  xid::new().to_string()
}

#[napi]
pub fn many(env: Env, size: u32) -> Array {
  let mut result = env.create_array(size).unwrap();
  for i in 0..size {
    result.set::<String>(i, xid::new().to_string()).unwrap();
  }
  result
}

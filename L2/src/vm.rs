use std::convert::Infallible;

use revm::{
  db::EmptyDBTyped, Context
};

pub fn create_evm_instance() -> Context<(), EmptyDBTyped<Infallible>> {
  Context::new_empty()
}
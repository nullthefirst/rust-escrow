#![no_std]

use soroban_sdk::{
  contract, contracttype, contractimlp, Env, String, log,
};

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
  pub job_id: String,
  pub amount: i128,
  pub released: bool,
}

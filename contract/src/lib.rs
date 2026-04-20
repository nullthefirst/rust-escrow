#![no_std]

use soroban_sdk::{
  contract, contracttype, contractimpl, Env, String, log,
};

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
  pub job_id: String,
  pub amount: i128,
  pub released: bool,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
  // get state
  pub fn get(env: Env, escrow_id: String) -> Escrow {
    env.storage().instance().get(&escrow_id).unwrap()
  }

  // create escrow
  pub fn create_escrow(env: Env, escrow_id: String, job_id: String, amount: i128) {
    let escrow = Escrow {
      job_id,
      amount,
      released: false,
    };

    env.storage().instance().set(&escrow_id, &escrow);

    log!(&env, "ESCROW_CREATED")
  }

  // release funds
  pub fn release_funds(env: Env, escrow_id: String) {
    let mut escrow: Escrow = env.storage().instance().get(&escrow_id).unwrap();

    if escrow.released {
      panic!("Already released!");
    }

    escrow.released = true;

    env.storage().instance().set(&escrow_id, &escrow);

    log!(&env, "ESCROW_RELEASED")
  }
}

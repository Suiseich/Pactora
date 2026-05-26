#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    Created,
    Funded,
    Approved,
    Released,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub token: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}

#[contracttype]
pub enum DataKey {
    NextEscrowId,
    Escrow(u64),
}

#[contract]
pub struct PactoraContract;

#[contractimpl]
impl PactoraContract {
    // Creates a project escrow record before funds are locked.
    pub fn create_escrow(
        env: Env,
        client: Address,
        freelancer: Address,
        token: Address,
        amount: i128,
    ) -> u64 {
        client.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let id = Self::next_id(&env);
        let escrow = Escrow {
            id,
            client,
            freelancer,
            token,
            amount,
            status: EscrowStatus::Created,
        };

        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
        env.storage()
            .persistent()
            .set(&DataKey::NextEscrowId, &(id + 1));

        id
    }

    // Moves the agreed token amount from the client into this contract.
    pub fn fund_escrow(env: Env, escrow_id: u64) {
        let mut escrow = Self::load_escrow(&env, escrow_id);
        escrow.client.require_auth();

        if escrow.status != EscrowStatus::Created {
            panic!("escrow is not ready to fund");
        }

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(
            &escrow.client,
            &env.current_contract_address(),
            &escrow.amount,
        );

        escrow.status = EscrowStatus::Funded;
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Marks the freelancer's work as accepted by the client.
    pub fn approve_work(env: Env, escrow_id: u64) {
        let mut escrow = Self::load_escrow(&env, escrow_id);
        escrow.client.require_auth();

        if escrow.status != EscrowStatus::Funded {
            panic!("escrow must be funded before approval");
        }

        escrow.status = EscrowStatus::Approved;
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Releases the locked funds to the freelancer after client approval.
    pub fn release_payment(env: Env, escrow_id: u64) {
        let mut escrow = Self::load_escrow(&env, escrow_id);

        if escrow.status != EscrowStatus::Approved {
            panic!("escrow must be approved before release");
        }

        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(
            &env.current_contract_address(),
            &escrow.freelancer,
            &escrow.amount,
        );

        escrow.status = EscrowStatus::Released;
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Cancels a funded or unfunded escrow before approval and refunds the client if needed.
    pub fn cancel_escrow(env: Env, escrow_id: u64) {
        let mut escrow = Self::load_escrow(&env, escrow_id);
        escrow.client.require_auth();

        if escrow.status != EscrowStatus::Created && escrow.status != EscrowStatus::Funded {
            panic!("escrow cannot be cancelled");
        }

        if escrow.status == EscrowStatus::Funded {
            let token_client = token::Client::new(&env, &escrow.token);
            token_client.transfer(
                &env.current_contract_address(),
                &escrow.client,
                &escrow.amount,
            );
        }

        escrow.status = EscrowStatus::Cancelled;
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Returns the full escrow record so a freelancer can verify locked funds and status.
    pub fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        Self::load_escrow(&env, escrow_id)
    }

    fn next_id(env: &Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::NextEscrowId)
            .unwrap_or(1)
    }

    fn load_escrow(env: &Env, escrow_id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("escrow not found"))
    }
}

#[cfg(test)]
mod test;

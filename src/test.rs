use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token, Address, Env, IntoVal,
};

fn setup() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let pactora_id = env.register_contract(None, PactoraContract);
    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_contract.address());

    token_admin_client.mint(&client, &1_000);

    (env, pactora_id, client, freelancer, token_contract.address())
}

#[test]
fn happy_path_releases_payment_to_freelancer() {
    let (env, pactora_id, client, freelancer, token_id) = setup();
    let pactora = PactoraContractClient::new(&env, &pactora_id);
    let token_client = token::Client::new(&env, &token_id);
    let amount = 100;

    let escrow_id = pactora.create_escrow(&client, &freelancer, &token_id, &amount);
    pactora.fund_escrow(&escrow_id);
    pactora.approve_work(&escrow_id);
    pactora.release_payment(&escrow_id);

    assert_eq!(token_client.balance(&freelancer), amount);
    assert_eq!(
        pactora.get_escrow(&escrow_id).status,
        EscrowStatus::Released
    );
}

#[test]
#[should_panic]
fn non_client_cannot_approve_work() {
    let (env, pactora_id, client, freelancer, token_id) = setup();
    let pactora = PactoraContractClient::new(&env, &pactora_id);
    let amount = 100;
    let stranger = Address::generate(&env);

    let escrow_id = pactora.create_escrow(&client, &freelancer, &token_id, &amount);
    pactora.fund_escrow(&escrow_id);

    env.mock_auths(&[MockAuth {
        address: &stranger,
        invoke: &MockAuthInvoke {
            contract: &pactora_id,
            fn_name: "approve_work",
            args: (escrow_id,).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    pactora.approve_work(&escrow_id);
}

#[test]
fn storage_updates_after_funding_and_approval() {
    let (env, pactora_id, client, freelancer, token_id) = setup();
    let pactora = PactoraContractClient::new(&env, &pactora_id);
    let amount = 250;

    let escrow_id = pactora.create_escrow(&client, &freelancer, &token_id, &amount);
    assert_eq!(pactora.get_escrow(&escrow_id).status, EscrowStatus::Created);

    pactora.fund_escrow(&escrow_id);
    assert_eq!(pactora.get_escrow(&escrow_id).status, EscrowStatus::Funded);

    pactora.approve_work(&escrow_id);
    let escrow = pactora.get_escrow(&escrow_id);
    assert_eq!(escrow.status, EscrowStatus::Approved);
    assert_eq!(escrow.amount, amount);
    assert_eq!(escrow.client, client);
    assert_eq!(escrow.freelancer, freelancer);
}

#[test]
fn client_can_cancel_funded_escrow_and_receive_refund() {
    let (env, pactora_id, client, freelancer, token_id) = setup();
    let pactora = PactoraContractClient::new(&env, &pactora_id);
    let token_client = token::Client::new(&env, &token_id);
    let amount = 300;
    let starting_balance = token_client.balance(&client);

    let escrow_id = pactora.create_escrow(&client, &freelancer, &token_id, &amount);
    pactora.fund_escrow(&escrow_id);
    pactora.cancel_escrow(&escrow_id);

    assert_eq!(token_client.balance(&client), starting_balance);
    assert_eq!(token_client.balance(&freelancer), 0);
    assert_eq!(
        pactora.get_escrow(&escrow_id).status,
        EscrowStatus::Cancelled
    );
}

#[test]
#[should_panic]
fn cannot_release_before_approval_or_funding() {
    let (env, pactora_id, client, freelancer, token_id) = setup();
    let pactora = PactoraContractClient::new(&env, &pactora_id);
    let amount = 100;

    let escrow_id = pactora.create_escrow(&client, &freelancer, &token_id, &amount);

    pactora.release_payment(&escrow_id);
}

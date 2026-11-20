//! Tests for treasury fee rounding over small successful valuation gains (#771).
//!
//! Validates that small recurring gains (1, 3, 5, 7, 10 stroops/units) have deterministic,
//! reproducible treasury performance fee outcomes and exact fee share / total supply deltas.

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_treasury_fee_rounding_small_valuation_gains() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VaultContract, ());
    let client = VaultContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let manager = Address::generate(&env);
    let treasury = Address::generate(&env);
    let user = Address::generate(&env);
    let asset = Address::generate(&env);

    client.initialize(
        &admin,
        &manager,
        &asset,
        &10_000_000,
        &10_000_000_000,
        &treasury,
        &1000, // 10% fee (1000 bps)
    );

    let initial_deposit: i128 = 100_000_000;
    client.deposit(&user, &initial_deposit);

    let initial_shares = client.balance(&user);
    let initial_total_shares = client.total_shares();
    assert_eq!(initial_shares, initial_deposit);
    assert_eq!(initial_total_shares, initial_deposit);

    let test_gains: [i128; 5] = [1, 3, 5, 7, 10];
    let mut expected_total_shares = initial_total_shares;

    for gain in test_gains {
        let shares_before = client.total_shares();
        let treasury_before = client.balance(&treasury);

        client.report_valuation_gain(&gain);

        let shares_after = client.total_shares();
        let treasury_after = client.balance(&treasury);

        let expected_fee_shares = (gain * 1000) / 10_000;
        let fee_share_delta = treasury_after - treasury_before;
        let total_supply_delta = shares_after - shares_before;

        assert_eq!(fee_share_delta, expected_fee_shares);
        assert_eq!(total_supply_delta, expected_fee_shares);
        expected_total_shares += expected_fee_shares;
        assert_eq!(shares_after, expected_total_shares);
    }
}

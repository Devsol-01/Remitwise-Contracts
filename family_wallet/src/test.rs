#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Address, Env, String};

#[test]
fn test_add_member() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);
    
    let address = Address::generate(&env);
    let name = String::from_str(&env, "Ravi");
    let role = String::from_str(&env, "sender");
    
    let result = client.add_member(&address, &name, &10000i128, &role);
    assert!(result);
}

#[test]
fn test_get_member() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);
    
    let address = Address::generate(&env);
    let name = String::from_str(&env, "Ravi");
    let role = String::from_str(&env, "sender");
    
    client.add_member(&address, &name, &10000i128, &role);
    
    let member = client.get_member(&address);
    assert!(member.is_some());
    let member = member.unwrap();
    assert_eq!(member.name, name);
    assert_eq!(member.spending_limit, 10000);
}

#[test]
fn test_check_spending_limit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);
    
    let address = Address::generate(&env);
    let name = String::from_str(&env, "Ravi");
    let role = String::from_str(&env, "sender");
    
    client.add_member(&address, &name, &10000i128, &role);
    
    assert!(client.check_spending_limit(&address, &5000i128));
    assert!(!client.check_spending_limit(&address, &15000i128));
}


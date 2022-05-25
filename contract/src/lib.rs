use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<String, String>,
}

impl Default for Welcome {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl Welcome {
    pub fn set_greeting(&mut self, name: String) {
        let account_id = env::signer_account_id();

        env::log(format!("Saving name '{}' for account '{}'", name, account_id,).as_bytes());

        self.records.insert(&account_id, &name);
    }

    pub fn get_greeting(&self, account_id: String) -> String {
        match self.records.get(&account_id) {
            Some(name) => "Hello ".to_string() + &name + &"!".to_string(),
            None => "Hello anonymous".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("Bob".to_string());
        assert_eq!(
            "Hello Bob!".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        assert_eq!(
            "Hello NEAR user".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}

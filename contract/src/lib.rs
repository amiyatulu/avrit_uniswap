use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{ext_contract,  AccountId, Balance, Gas, Promise};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Uniswap {
    records: UnorderedMap<String, String>,
    uni_totalsupply: u128,
    uni_balances:LookupMap<AccountId, Balance>,
    avrit_token_balance: u128,

}

#[near_bindgen]
impl Uniswap {

    pub fn add_liquidity(&mut self, min_liquidity: u128, max_tokens: u128) {
        let deposit = env::attached_deposit();
        let contract_near_balance = env::account_balance();
        let user_address = env::predecessor_account_id();
        assert!(max_tokens > 0, "Maximum tokens should be greater than zero");
        assert!(deposit > 0, "Deposit must be greater than zero");
        let total_liquidity = self.uni_totalsupply;
        if(total_liquidity > 0) {
            assert!(min_liquidity > 0, "Minimum liquidity must be greater than zero");
            let near_reserve =  contract_near_balance - deposit;
            let token_reserve = self.avrit_token_balance;
            let token_amount = deposit * token_reserve/near_reserve + 1;
            let liquidity_minted = deposit * total_liquidity/near_reserve;
            assert!(max_tokens >= token_amount, "max_tokens must be greater than token amount");
            assert!(liquidity_minted >= min_liquidity, "liquidity minted should be greater or equal to min_liquidity");
            let balance_option = self.uni_balances.get(&user_address);
            match balance_option {
                Some(balance) => {
                    self.uni_balances.insert(&user_address, &(balance+ liquidity_minted));
                }
                None => {
                    self.uni_balances.insert(&user_address, &liquidity_minted);
                }
            }
            self.uni_totalsupply = total_liquidity + liquidity_minted;

        }
    }



    pub fn set_status(&mut self, message: String) {
        env::log(b"A");
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        env::log(b"A");
        return self.records.get(&account_id);
    }

    #[init]    
    pub fn new() -> Self {
        assert!(!env::state_exists(), "ERR_CONTRACT_IS_INITIALIZED");
        let id = "68dbf390-0b13-4db1-bb7d-9bf6ac5d23ab".to_string().into_bytes();
        Self{
            records:UnorderedMap::new(id),
            uni_totalsupply: 0,
            uni_balances: LookupMap::new(b"9a0e582c".to_vec()),
            avrit_token_balance: 0,

        }
    }
}

impl Default for Uniswap {
    fn default() -> Self {
        panic!("StatusMessage should be initialized before usage")
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Uniswap::new();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Uniswap::new();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
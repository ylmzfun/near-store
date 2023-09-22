use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Balance, near_bindgen};
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U128};


const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Goods {
    pub premium: bool,
    pub sender: AccountId,
    pub num: f32,
    pub text: String,
    pub price: f32,
    pub fee: f32
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Store {
    goods_list: Vector<Goods>,
}


impl Default for Store {
    // The default trait with which to initialize the contract
    fn default() -> Self {
        Self{goods_list: Vector::new(b"m")}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Store {
    // Public: Returns the stored greeting, defaulting to 'Hello'
    // 读取购物车列表
    pub fn get_shoppingcar(&self,from_index:Option<U128>, limit:Option<u64>) -> Vec<Goods> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.goods_list.iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .collect()
    }

    // Public: Takes a greeting, such as 'howdy', and records it
    //添加到购物车
    #[payable]
    pub fn set_shoppingcar(&mut self, text: String, num:f32, price:f32) {
        // Record a log permanently to the blockchain
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();
        let fee = num * price;
        let goodss = Goods{premium,sender,num,text, price, fee};
        self.goods_list.push(&goodss)
    }

    pub fn get_total(&self) -> u64 {self.goods_list.len()}
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_shoppingcar() {
        let mut contract = Store::default();
        contract.set_shoppingcar("商品A".to_string(),1.0,2.0f32);

        let posted_message = &contract.get_shoppingcar(None, None)[0];
        assert_eq!(posted_message.num, 1.0);
        assert_eq!(posted_message.text, "商品A".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Store::default();
        contract.set_shoppingcar("1st message".to_string(),1.0,1.0);
        contract.set_shoppingcar("2nd message".to_string(),2.0,2.0);
        contract.set_shoppingcar("3rd message".to_string(),3.0,3.0);

        let total = &contract.get_total();
        assert!(*total == 3);

        let last_message = &contract.get_shoppingcar(Some(U128::from(1)), Some(2))[1];
        assert_eq!(last_message.num, 3.0);
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}

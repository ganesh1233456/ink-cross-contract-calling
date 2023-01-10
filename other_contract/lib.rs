#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// pub use self::other_contract::OtherContract;

pub use self::other_contract::{
    OtherContract,
    OtherContractRef,
};

#[ink::contract]
pub mod other_contract {
    /// Storage for the other contract.
    #[ink(storage)]
    pub struct OtherContract {
        value: i32,
    }

    impl OtherContract {
        /// Initializes the contract.
        #[ink(constructor)]
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        /// Returns the current state.
        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            self.value
        }
    }
}

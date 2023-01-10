#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod MyContract {

    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct MyContract {
        /// The other contract.
        other_contract: OtherContractRef,
    }

    impl MyContract {
        /// Instantiate `MyContract with the given
        /// sub-contract codes and some initial value.
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash, version: u32) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_be_bytes();
            let other_contract = OtherContractRef::new(1337)
                .endowment(total_balance / 4)
                .code_hash(other_contract_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the OtherContract contract: {:?}",
                        error
                    )
                });
            Self { other_contract }
        }

        /// Calls the other contract.
        #[ink(message)]
        pub fn call_other_contract(&self) -> i32 {
            self.other_contract.get_value()
        }
    }
}

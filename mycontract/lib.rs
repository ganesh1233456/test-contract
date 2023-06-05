#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            return Err($y.into());
        }
    }};
}

#[openbrush::contract]
mod mycontract {
    use openbrush::{contracts::psp22::*, traits::Storage};

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Mycontract {
        value: bool,
        psp22_contract: AccountId,
        owner: AccountId,
    }

    #[derive(scale::Decode, scale::Encode, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum MyContractError {
        MustBeGreaterThanZero,
        NotEnoughBalance,
    }

    impl Mycontract {
        #[ink(constructor)]
        pub fn new(init_value: bool, psp22_contract: AccountId) -> Self {
            let owner = Self::env().caller();
            Self {
                value: init_value,
                psp22_contract,
                owner,
            }
        }

        #[ink(message, payable)]
        pub fn flip(&mut self) -> Result<(), MyContractError>{
            let caller = self.env().caller();
            let psp22_balance = PSP22Ref::balance_of(&self.psp22_contract, caller);
            let value = self.env().transferred_value();

            ensure!(value > 0, MyContractError::MustBeGreaterThanZero);
            ensure!(psp22_balance > 0, MyContractError::NotEnoughBalance);

            self.value = !self.value;
            self.env()
                .transfer(self.owner, psp22_balance)
                .unwrap_or_default();

            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

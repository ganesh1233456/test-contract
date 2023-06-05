#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod myapp {

    // imports from openbrush
    use openbrush::contracts::psp22::extensions::mintable::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyApp {
        #[storage_field]
        psp22: psp22::Data,
        cap: Balance,
    }

    // Section contains default implementation without any modifications
    impl PSP22 for MyApp {}
    impl PSP22Mintable for MyApp {}

    impl MyApp {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            _instance
                ._mint_to(_instance.env().caller(), initial_supply)
                .expect("Should mint");
            _instance
        }
    }
}

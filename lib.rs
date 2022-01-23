#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod my_erc20 {
    use ink_storage::{
        collections::HashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    pub struct MyErc20 {
        total_supply: Lazy<Balance>,
        balances: HashMap<AccountId, Balance>,
        allowances: HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }    

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }  

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]    
    pub enum Error{
        InsufficientBalance,
        InsufficientAproval,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl MyErc20 {

        #[ink(constructor)]
        pub fn new(supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances=HashMap::new();
            balances.insert(caller, supply);

            Self{
                total_supply: Lazy::new(supply),
                balances,
                allowances: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }        

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance{
            self.balances.get(&owner).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn allowances(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        }        

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.inner_transfer(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }        

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowances(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAproval)
            }
            self.inner_transfer(from, to, value)?;
            self.allowances
                .insert((from, caller), allowance - value);
            Ok(())
        }        

        pub fn inner_transfer(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }

            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }        
    }

}

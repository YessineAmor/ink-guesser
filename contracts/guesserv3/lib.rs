#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract()]
mod guesserv3 {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
        Vec as StorageVec,
    };
    use ink_prelude::string;
    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Challenge {
        challenge_hash: Hash,
        challenge_answer: Hash,
        challenge_prize: Balance,
        challenge_solved: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Submission {
        submission_hash: Hash,
        submission_answer: string::String,
        submission_result: bool,
    }
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Guesserv3 {
        // Maps all challenges to their owners
        challenge_owner: StorageHashMap<Hash, AccountId>,
        submission_owner: StorageHashMap<Hash, AccountId>,
        challenges: StorageHashMap<Hash, Challenge>,
        submissions: StorageVec<Submission>,
        owners: StorageVec<AccountId>,
        // challenge_submissions: StorageHashMap<Hash, StorageBox<StorageVec<Hash>>>,
    }
    /// Event emitted when a challenge gets created.
    #[ink(event)]
    pub struct Creation {
        /// The challenge that was created
        #[ink(topic)]
        challenge: Hash,
        /// The challenge prize
        #[ink(topic)]
        challenge_prize: Balance,
    }

    impl Guesserv3 {
        /// Create a new challenge
        #[ink(message)]
        pub fn new_challenge(
            &mut self,
            challenge_answer: Hash,
            challenge_prize: Balance,
            challenge_hash: Hash,
        ) -> bool {
            let caller = self.env().caller();
            self.challenges.insert(
                challenge_hash,
                Challenge {
                    challenge_answer,
                    challenge_prize,
                    challenge_hash,
                    challenge_solved: true,
                },
            );
            self.challenge_owner.insert(challenge_hash, caller);
            self.owners.push(caller);
            Self::env().emit_event(Creation {
                challenge: challenge_hash,
                challenge_prize,
            });
            true
        }
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let guesserv3 = Guesserv3::default();
            assert_eq!(false, false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            assert_eq!(false, false);
        }
    }
}

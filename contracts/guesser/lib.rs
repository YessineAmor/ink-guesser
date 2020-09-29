#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod guesser {
    
    use ink_core::storage;
    use ink_prelude::string;

    #[derive(Default)]
    pub struct Challenge {
        challenge_hash: storage::Value<Hash>,
        challenge_answer: storage::Value<Hash>,
        challenge_prize: Balance,
        challenge_solved: bool,
    }

    pub struct Submission {
        submission_hash: storage::Value<Hash>,
        submission_answer: string::String,
        submission_result: storage::Value<bool>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Guesser {
        /// Maps all challenges to their owners
        challenge_owner: storage::HashMap<(Hash, AccountId)>,
        submission_owner: storage::HashMap<(Hash, AccountId)>,
        challenges: storage::Vec<Challenge>,
        owners: storage::Vec<AccountId>,
        challenge_submissions: storage::HashMap<(Hash, storage::Vec<Hash>)>,
    }
    /// Event emitted when a challenge gets created.
    #[ink(event)]
    struct Creation {
        /// The challenge that was created
        #[ink(topic)]
        challenge: Hash,
        /// The challenge prize
        #[ink(topic)]
        challenge_prize: Balance,
    }
    impl Guesser {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        fn new_challenge(
            &mut self,
            challenge_answer: Hash,
            challenge_prize: Balance,
            challenge_hash: Hash,
        ) -> bool {
            let caller = self.env().caller();
            self.challenges.push(Challenge {
                challenge_answer,
                challenge_hash,
                challenge_prize,
                challenge_solved: false,
            });
            match self.owners.get(caller) {
                Some(_) => {}
                None => {
                    self.owners.push(caller);
                }
            }
            self.challenge_owner.insert(challenge_hash, caller);
            self.env().emit_event(Creation {
                challenge: challenge_hash,
                challenge_prize,
            });
            true
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        fn default(&mut self) {
            self.new_challenge(challenge_answer, challenge_prize, challenge_hash)
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get_challenges(&self, owner: &AccountId) -> storage::Vec<Challenge> {
            *self.challenges
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
            // Note that even though we defined our `#[ink(constructor)]`
            // above as `&mut self` functions that return nothing we can call
            // them in test code as if they were normal Rust constructors
            // that take no `self` argument but return `Self`.
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {}
    }
}

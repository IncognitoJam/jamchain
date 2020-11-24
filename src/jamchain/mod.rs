use std::collections::HashMap;

/// The actual Blockchain container
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Stores all the blocks which are accepted already within the blockchain.
    pub blocks: Vec<Block>,

    /// Lookup from AccountID (will be a public key later) to Account.
    /// Effectively, this represents the WorldState.
    pub accounts: HashMap<String, Account>,

    /// Will store transactions which should be added to the chain but aren't
    /// yet.
    pending_transactions: Vec<Transaction>,
}

/// Represents the current state of the blockchain after all Blocks are
/// executed. A world state is technically not necessary since we could always
/// build the information by iterating through all the blocks. Generally, this
/// doesn't seem like a good option. However, we do not force the actual
/// Blockchain to implement a WorldState but rather behave like having one.
/// This trait just defines an expected interface into our Blockchain.
trait WorldState {
    /// Return all registered user ids.
    fn get_user_ids(&self) -> Vec<String>;

    /// Return the account with the matching id, if it's available (mutable).
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Return the account with the matching id, if it's available.
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Add a new account.
    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>;
}

/// TODO: complete definitions
#[derive(Debug, Clone)]
pub struct Account;

#[derive(Debug, Clone)]
pub struct AccountType;

#[derive(Debug, Clone)]
pub struct Block;

#[derive(Debug, Clone)]
pub struct Transaction;

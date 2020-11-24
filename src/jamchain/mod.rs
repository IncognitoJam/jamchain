use std::collections::HashMap;
use std::time::SystemTime;

/// The actual Blockchain container
#[derive(Clone, Debug)]
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

/// Represents an account on the blockchain. This is the important part of the
/// [WorldState] of the chain. The final state of each account is determined
/// after processing all of the [Block]s and their [Transactions]s within.
#[derive(Clone, Debug)]
pub struct Account {
    /// An account can store any information in a dictionary.
    store: HashMap<String, String>,

    /// Is this a user account or something else.
    acc_type: AccountType,

    /// The amount of tokens the user has, like currency.
    tokens: u128,
}

/// The chain can support different types of account which could be used to
/// represent different roles in the system, like users, contracts or automated
/// events by the chain itself.
#[derive(Clone, Debug)]
pub enum AccountType {
    /// A common user account.
    User,
    /// An account which doesn't represent an individual, but a smart contract.
    Contract,
}

#[derive(Clone, Debug)]
pub struct Block {
    /// Actions that this block includes. There has to be at least one in the
    /// block.
    pub(crate) transactions: Vec<Transaction>,

    /// Hash of the previous block, connecting them together.
    prev_hash: Option<String>,

    /// Hash of the current block. The hashed data includes the hash of the
    /// previous block, building a chain which is resistant to tampering.
    hash: Option<String>,

    /// Some arbitrary number which will be used for PoW.
    nonce: u128,
}

/// Stores an action to take place on the blockchain.
#[derive(Clone, Debug)]
pub struct Transaction {
    /// Unique number (prevents replay attacks).
    nonce: u128,

    /// Source account ID.
    from: String,

    /// Stores the time the transaction was created.
    created_at: SystemTime,

    /// The type of the transaction and it's additional information.
    pub(crate) record: TransactionData,

    /// Signature of the hash of the whole message.
    signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionData {}

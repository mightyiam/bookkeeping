#[macro_use]
extern crate maplit;
/// The various entities involved in accounting
pub mod entities {
    use crate::book::AccountKey;
    use std::collections::HashMap;
    use steel_cent::{currency, currency::Currency, Money};
    /// Represents money either taken out of or put into an account.
    ///
    /// A group of multiple moves can make up a [transaction](crate::entities::Transaction).
    #[derive(Eq, PartialEq, Debug, Clone)]
    pub struct Move {
        account_key: AccountKey,
        money: Money,
    }
    impl Move {
        /// Creates a new move.
        ///
        /// ```
        /// use envelope_system::entities::{ Move, MoveInput };
        /// use envelope_system::{ currency, Money };
        /// use envelope_system::book::{ AccountKey };
        /// // Imagine that you got this key from a `Book`
        /// let account_key = AccountKey::default();
        /// let money = Money::of_major(currency::THB, 10);
        /// let move_ = Move::new(MoveInput{ account_key, money });
        /// ```
        pub fn new(input: MoveInput) -> Self {
            let MoveInput { account_key, money } = input;
            Self { account_key, money }
        }
    }
    /// Input for creating a new move.
    pub struct MoveInput {
        /// Used to reference an [account](Account) in the [book](crate::book::Book).
        pub account_key: AccountKey,
        /// Specifies the money moved.
        pub money: Money,
    }
    /// Represents an incomplete transaction that may yet be imbalanced.
    ///
    /// Entities of this type may be finalized into [transaction](Transaction)s using [TransactionDraft::finalize].
    ///
    /// ```
    /// use envelope_system::entities::{TransactionDraft, TransactionDraftInput, Move, MoveInput, Transaction};
    /// use envelope_system::book::{AccountKey};
    /// use envelope_system::{ currency, Money };
    /// let mut draft = TransactionDraft::new(TransactionDraftInput{});
    /// // Imagine you got this key from a `Book`.
    /// let atm = AccountKey::default();
    /// let money = Money::of_major(currency::THB, -10);
    /// draft.add_move(Move::new(MoveInput{account_key: atm, money }));
    /// // Imagine you got this key from a `Book`, as well.
    /// let wallet = AccountKey::default();
    /// let money = Money::of_major(currency::THB, 10);
    /// draft.add_move(Move::new(MoveInput{account_key: wallet, money }));
    /// let transaction: Transaction = draft.finalize();
    /// ```
    pub struct TransactionDraft {
        moves: Vec<Move>,
    }

    #[test]
    fn move_new() {
        let money = Money::of_major(currency::THB, 20);
        let money_clone = money.clone();
        let move_ = Move::new(MoveInput {
            money,
            account_key: AccountKey::default(),
        });
        assert_eq!(move_.money, money_clone);
        assert_eq!(move_.account_key, AccountKey::default());
    }

    impl TransactionDraft {
        /// New transaction drafts start with no moves.
        /// Moves are to be added using [TransactionDraft::add_move].
        pub fn new(input: TransactionDraftInput) -> Self {
            Self { moves: Vec::new() }
        }
        /// Adds a move to the transaction draft.
        pub fn add_move(&mut self, move_: Move) {
            self.moves.push(move_);
        }
        /// Calculates the balances from the moves of the dransaction draft, one balance per currency.
        // TODO: the key should be `Currency`
        pub fn balances(&self) -> HashMap<&Currency, Money> {
            self.moves.iter().fold(HashMap::new(), |mut acc, curr| {
                let money = &curr.money;
                let new_balance = acc
                    .get(&money.currency)
                    .map_or(money.clone(), |balance| balance.clone() + money.clone());
                acc.insert(&money.currency, new_balance);
                acc
            })
        }
        /// Figures out whether all of the balances are zero.
        pub fn are_balanced(balances: &HashMap<&Currency, Money>) -> bool {
            balances
                .iter()
                .all(|(_, balance)| balance == &Money::zero(balance.currency))
        }
        /// If all of the balances are found to be zero, the draft will be finalized.
        ///
        /// ## Panics
        /// If any of the balances are not zero.
        pub fn finalize(self) -> Transaction {
            if Self::are_balanced(&self.balances()) {
                Transaction { moves: self.moves }
            } else {
                panic!("Transaction draft not balanced.");
            }
        }
    }
    #[test]
    fn transaction_draft_new() {
        let draft = TransactionDraft::new(TransactionDraftInput {});
        assert_eq!(draft.moves.len(), 0);
    }
    #[test]
    fn transaction_draft_add_move() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        let move_ = Move::new(MoveInput {
            money: Money::of_major(currency::THB, -10),
            account_key: AccountKey::default(),
        });
        let move_clone = move_.clone();
        draft.add_move(move_);
        assert_eq!(draft.moves.len(), 1);
        assert_eq!(draft.moves[0], move_clone);
        let move_ = Move::new(MoveInput {
            money: Money::of_major(currency::THB, 10),
            account_key: AccountKey::default(),
        });
        let move_clone = move_.clone();
        draft.add_move(move_);
        assert_eq!(draft.moves.len(), 2);
        assert_eq!(draft.moves[1], move_clone);
    }
    #[test]
    fn transaction_draft_balances() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        let move_ = Move::new(MoveInput {
            money: Money::of_major(currency::THB, -10),
            account_key: AccountKey::default(),
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(
            balances[&currency::THB],
            Money::of_major(currency::THB, -10)
        );
        let move_ = Move::new(MoveInput {
            money: Money::of_major(currency::THB, 10),
            account_key: AccountKey::default(),
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[&currency::THB], Money::zero(currency::THB));
        let move_ = Move::new(MoveInput {
            money: Money::of_major(currency::ILS, 20),
            account_key: AccountKey::default(),
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[&currency::THB], Money::zero(currency::THB));
        assert_eq!(balances[&currency::ILS], Money::of_major(currency::ILS, 20));
    }
    #[test]
    fn transaction_draft_are_balanced() {
        let balances = hashmap! {};
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
        let balances = hashmap! {
            &currency::THB => Money::zero(currency::THB),
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
        let balances = hashmap! {
            &currency::THB => Money::of_major(currency::THB, 1),
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &currency::THB => Money::of_major(currency::THB, -1),
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &currency::THB => Money::zero(currency::THB),
            &currency::ILS => Money::of_major(currency::ILS, 2),
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &currency::THB => Money::zero(currency::THB),
            &currency::ILS => Money::zero(currency::ILS),
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
    }
    #[test]
    #[should_panic(expected = "Transaction draft not balanced.")]
    fn finalize_unbalanced_transaction_draft() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        draft.add_move(Move::new(MoveInput {
            money: Money::of_major(currency::GBP, 100),
            account_key: AccountKey::default(),
        }));
        draft.finalize();
    }
    #[test]
    fn finalize_balanced_transaction_draft() {
        let draft = TransactionDraft::new(TransactionDraftInput {});
        let _transaction: Transaction = draft.finalize();
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        draft.add_move(Move::new(MoveInput {
            money: Money::of_major(currency::GBP, 100),
            account_key: AccountKey::default(),
        }));
        draft.add_move(Move::new(MoveInput {
            money: Money::of_major(currency::GBP, -100),
            account_key: AccountKey::default(),
        }));
        let _transaction: Transaction = draft.finalize();
    }
    /// Input for creating a new transaction draft.
    pub struct TransactionDraftInput {}
    /// A group of related [move](crate::entities::Move)s that all occur at some time.
    ///
    /// Transactions cannot be created directly.
    /// They start as [draft](crate::entities::TransactionDraft)s.
    pub struct Transaction {
        moves: Vec<Move>,
    }
    /// Represents an account. For example, a bank account or a wallet.
    ///
    /// ## Example:
    /// ```
    /// use envelope_system::entities::{Account, AccountInput};
    /// let wallet = Account::new(AccountInput{ name: String::from("Wallet") });
    /// ```
    #[derive(PartialEq, Debug)]
    pub struct Account {
        name: String,
    }
    /// Input for creating a new account.
    pub struct AccountInput {
        /// The name of the account.
        pub name: String,
    }
    impl Account {
        /// Creates a new account.
        pub fn new(input: AccountInput) -> Self {
            Self { name: input.name }
        }
    }
}
mod changes {
    use crate::book::{Book, ChangeApplicationFailure};
    use crate::entities;
    use steel_cent::currency::Currency;
    pub trait Change {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure>;
    }
    pub struct AddCurrencyInput {
        pub currency: &'static Currency,
    }
    pub struct AddCurrency {
        pub(crate) currency: &'static Currency,
    }
    impl AddCurrency {
        pub fn new(input: AddCurrencyInput) -> Self {
            Self {
                currency: input.currency,
            }
        }
    }
    impl Change for AddCurrency {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure> {
            if book.currencies.contains(self.currency) {
                Err(ChangeApplicationFailure::CurrencyAlreadyExists(
                    self.currency,
                ))
            } else {
                book.currencies.insert(self.currency);
                Ok(())
            }
        }
    }
    pub struct AddAccountInput {
        pub account: entities::Account,
    }
    pub struct AddAccount {
        pub(crate) account: entities::Account,
    }
    impl AddAccount {
        pub fn new(input: AddAccountInput) -> Self {
            Self {
                account: input.account,
            }
        }
    }
    impl Change for AddAccount {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure> {
            book.accounts.insert(self.account);
            Ok(())
        }
    }
}
pub mod book {
    use crate::changes::Change;
    use crate::entities;
    use slotmap::{new_key_type, DenseSlotMap};
    use std::collections::{HashMap, HashSet};
    use steel_cent::currency::Currency;
    new_key_type! {
        pub struct CurrencyKey;
        pub struct TransactionDraftKey;
        pub struct TransactionKey;
        pub struct AccountKey;
    }
    pub struct Book {
        pub(crate) currencies: HashSet<&'static Currency>,
        pub(crate) transaction_drafts:
            DenseSlotMap<TransactionDraftKey, entities::TransactionDraft>,
        pub(crate) transactions: DenseSlotMap<TransactionKey, entities::Transaction>,
        pub(crate) accounts: DenseSlotMap<AccountKey, entities::Account>,
    }
    impl Book {
        pub fn new() -> Self {
            Book {
                currencies: HashSet::new(),
                transaction_drafts: DenseSlotMap::with_key(),
                transactions: DenseSlotMap::with_key(),
                accounts: DenseSlotMap::with_key(),
            }
        }
        pub fn apply(&mut self, change: impl Change) -> Result<(), ChangeApplicationFailure> {
            change.apply(self)
        }
    }
    pub enum ChangeApplicationFailure {
        CurrencyAlreadyExists(&'static Currency),
    }
    #[cfg(test)]
    mod tests {
        use crate::book::Book;
        #[test]
        fn initial_state() {
            let book = Book::new();
            assert_eq!(book.currencies.len(), 0);
            assert_eq!(book.accounts.len(), 0);
            assert_eq!(book.transaction_drafts.len(), 0);
            assert_eq!(book.transactions.len(), 0);
        }
        mod changes {
            use crate::book::Book;
            use crate::changes;
            use crate::entities;
            use steel_cent::currency;
            #[test]
            fn change_add_currency() {
                let mut book = Book::new();
                book.apply(changes::AddCurrency::new(changes::AddCurrencyInput {
                    currency: &currency::THB,
                }));
                assert_eq!(book.currencies.len(), 1);
                assert!(book.currencies.contains(&currency::THB))
            }
            #[test]
            fn change_add_account() {
                let mut book = Book::new();
                let account = entities::Account::new(entities::AccountInput {
                    name: String::from("Wallet"),
                });
                book.apply(changes::AddAccount::new(changes::AddAccountInput {
                    account,
                }));
                assert_eq!(book.accounts.len(), 1);
                assert_eq!(
                    *book.accounts.iter().next().unwrap().1,
                    entities::Account::new(entities::AccountInput {
                        name: String::from("Wallet"),
                    })
                )
            }
        }
    }
}

pub use steel_cent::{currency, Currency, Money};

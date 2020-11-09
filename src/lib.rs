#[macro_use]
extern crate maplit;
/// The various entities involved in accounting
pub mod entities {
    use crate::book::{AccountKey, CurrencyKey};
    use slotmap::DenseSlotMap;
    use std::collections::HashMap;
    /// Represents a currency.
    ///
    /// ```
    /// use envelope_system::entities::*;
    /// let thb = Currency::new(CurrencyInput{ code: "THB".into(), decimal_places: 2 });
    /// ```
    pub struct Currency {
        pub(crate) code: String,
        pub(crate) decimal_places: u8,
    }
    impl Currency {
        pub fn new(input: CurrencyInput) -> Self {
            let CurrencyInput {
                code,
                decimal_places,
            } = input;
            Self {
                code,
                decimal_places,
            }
        }
    }
    impl PartialEq for Currency {
        fn eq(&self, other: &Self) -> bool {
            other.code == self.code
        }
    }
    /// Input for creating a new move.
    pub struct CurrencyInput {
        pub code: String,
        pub decimal_places: u8,
    }
    /// Represents money either taken out of or put into an account.
    ///
    /// A group of multiple moves can make up a [transaction](crate::entities::Transaction).
    #[derive(Eq, PartialEq, Debug, Clone)]
    pub struct Move {
        account_key: AccountKey,
        currency_key: CurrencyKey,
        amount: i64,
    }
    impl Move {
        /// Creates a new move.
        ///
        /// ```
        /// use envelope_system::entities::{ Move, MoveInput };
        /// use envelope_system::book::{ AccountKey, CurrencyKey };
        /// // Imagine that you got these keys from a `Book`
        /// let account_key = AccountKey::default();
        /// let currency_key = CurrencyKey::default();
        /// let move_ = Move::new(MoveInput{account_key, currency_key, amount: 2000 });
        /// ```
        pub fn new(input: MoveInput) -> Self {
            let MoveInput {
                account_key,
                currency_key,
                amount,
            } = input;
            Self {
                account_key,
                currency_key,
                amount,
            }
        }
    }
    /// Input for creating a new move.
    pub struct MoveInput {
        /// Used to reference an [account](Account) in the [book](crate::book::Book).
        pub account_key: AccountKey,
        /// Used to reference a [currency](Currency) in the [book](crate::book::Book) that represents the currency moved.
        pub currency_key: CurrencyKey,
        /// Specifies the amount moved, in minor unit of currency.
        pub amount: i64,
    }
    /// Represents an incomplete transaction that may yet be imbalanced.
    ///
    /// Entities of this type may be finalized into [transaction](Transaction)s using [TransactionDraft::finalize].
    ///
    /// ```
    /// use envelope_system::entities::{TransactionDraft, TransactionDraftInput, Move, MoveInput, Transaction};
    /// use envelope_system::book::{AccountKey, CurrencyKey};
    /// let mut draft = TransactionDraft::new(TransactionDraftInput{});
    /// // Imagine you got this key from a `Book`.
    /// let atm = AccountKey::default();
    /// let thb = CurrencyKey::default();
    /// draft.add_move(Move::new(MoveInput{account_key: atm, currency_key: thb, amount: -1000 }));
    /// // Imagine you got this key from a `Book`, as well.
    /// let wallet = AccountKey::default();
    /// draft.add_move(Move::new(MoveInput{account_key: wallet, currency_key: thb, amount: 1000 }));
    /// let transaction: Transaction = draft.finalize();
    /// ```
    pub struct TransactionDraft {
        moves: Vec<Move>,
    }

    #[test]
    fn move_new() {
        let amount = 2000;
        let move_ = Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount,
        });
        assert_eq!(move_.account_key, AccountKey::default());
        assert_eq!(move_.currency_key, CurrencyKey::default());
        assert_eq!(move_.amount, 2000);
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
        pub fn balances(&self) -> HashMap<&CurrencyKey, i64> {
            self.moves.iter().fold(HashMap::new(), |mut acc, curr| {
                let amount = curr.amount;
                let new_balance = acc
                    .get(&curr.currency_key)
                    .map_or(amount, |balance| balance + amount);
                acc.insert(&curr.currency_key, new_balance);
                acc
            })
        }
        /// Figures out whether all of the balances are zero.
        pub fn are_balanced(balances: &HashMap<&CurrencyKey, i64>) -> bool {
            balances.iter().all(|(_, balance)| *balance == 0)
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
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount: -1000,
        });
        let move_clone = move_.clone();
        draft.add_move(move_);
        assert_eq!(draft.moves.len(), 1);
        assert_eq!(draft.moves[0], move_clone);
        let move_ = Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount: 1000,
        });
        let move_clone = move_.clone();
        draft.add_move(move_);
        assert_eq!(draft.moves.len(), 2);
        assert_eq!(draft.moves[1], move_clone);
    }
    #[test]
    fn transaction_draft_balances() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        let mut sm = DenseSlotMap::with_key();
        let thb = sm.insert("obtain a key");
        let move_ = Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: thb,
            amount: -1000,
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[&thb], -1000,);
        let move_ = Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: thb,
            amount: 1000,
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[&thb], 0);
        let ils = sm.insert("obtain a key");
        let move_ = Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: ils,
            amount: 2000,
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[&thb], 0);
        assert_eq!(balances[&ils], 2000);
    }
    #[test]
    fn transaction_draft_are_balanced() {
        let mut sm: DenseSlotMap<CurrencyKey, &str> = DenseSlotMap::with_key();
        let thb = sm.insert("obtain a key");
        let ils = sm.insert("obtain a key");
        let balances = hashmap! {};
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
        let balances = hashmap! {
            &thb => 0,
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
        let balances = hashmap! {
            &thb => 100,
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &thb => -100,
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &thb => 0,
            &ils => 200,
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), false);
        let balances = hashmap! {
            &thb => 0,
            &ils => 0,
        };
        assert_eq!(TransactionDraft::are_balanced(&balances), true);
    }
    #[test]
    #[should_panic(expected = "Transaction draft not balanced.")]
    fn finalize_unbalanced_transaction_draft() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        draft.add_move(Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount: 10000,
        }));
        draft.finalize();
    }
    #[test]
    fn finalize_balanced_transaction_draft() {
        let draft = TransactionDraft::new(TransactionDraftInput {});
        let _transaction: Transaction = draft.finalize();
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        draft.add_move(Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount: 10000,
        }));
        draft.add_move(Move::new(MoveInput {
            account_key: AccountKey::default(),
            currency_key: CurrencyKey::default(),
            amount: -10000,
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
    use crate::{entities, entities::Currency};
    pub trait Change {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure>;
    }
    pub struct AddCurrencyInput {
        pub currency: Currency,
    }
    pub struct AddCurrency {
        pub(crate) currency: Currency,
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
            if book
                .currencies
                .values()
                .any(|currency| self.currency == *currency)
            {
                Err(ChangeApplicationFailure::CurrencyAlreadyExists(
                    self.currency.code,
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
    use crate::{entities, entities::Currency};
    use slotmap::{new_key_type, DenseSlotMap};
    use std::collections::{HashMap, HashSet};
    new_key_type! {
        pub struct CurrencyKey;
        pub struct TransactionDraftKey;
        pub struct TransactionKey;
        pub struct AccountKey;
    }
    pub struct Book {
        pub(crate) currencies: DenseSlotMap<CurrencyKey, Currency>,
        pub(crate) transaction_drafts:
            DenseSlotMap<TransactionDraftKey, entities::TransactionDraft>,
        pub(crate) transactions: DenseSlotMap<TransactionKey, entities::Transaction>,
        pub(crate) accounts: DenseSlotMap<AccountKey, entities::Account>,
    }
    impl Book {
        pub fn new() -> Self {
            Book {
                currencies: DenseSlotMap::with_key(),
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
        CurrencyAlreadyExists(String),
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
            use crate::entities::*;
            #[test]
            fn change_add_currency() {
                let mut book = Book::new();
                book.apply(changes::AddCurrency::new(changes::AddCurrencyInput {
                    currency: Currency::new(CurrencyInput {
                        code: "THB".into(),
                        decimal_places: 2,
                    }),
                }));
                assert_eq!(book.currencies.len(), 1);
                assert!(book.currencies.values().any(|cur| cur.code == "THB"))
            }
            #[test]
            fn change_add_account() {
                let mut book = Book::new();
                let account = Account::new(AccountInput {
                    name: String::from("Wallet"),
                });
                book.apply(changes::AddAccount::new(changes::AddAccountInput {
                    account,
                }));
                assert_eq!(book.accounts.len(), 1);
                assert_eq!(
                    *book.accounts.iter().next().unwrap().1,
                    Account::new(AccountInput {
                        name: String::from("Wallet"),
                    })
                )
            }
        }
    }
}

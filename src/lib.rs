#[macro_use]
extern crate maplit;
/// The various entities involved in accounting
pub mod entities {
    use std::collections::HashMap;
    /// Represents a currency.
    ///
    /// ```
    /// use envelope_system::entities::*;
    /// let thb = Currency::new(CurrencyInput{ code: "THB".into(), decimal_places: 2 });
    /// ```
    // TODO is deriving Eq correct here?
    #[derive(Eq, Clone, Debug)]
    pub struct Currency {
        pub(crate) code: String,
        pub(crate) decimal_places: u8,
    }
    impl Currency {
        /// Creates a new currency.
        ///
        /// ```
        /// use envelope_system::entities::*;
        /// let currency = Currency::new(CurrencyInput{ code: "THB".into(), decimal_places: 2 });
        /// ```
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
    use std::hash::{Hash, Hasher};
    impl Hash for Currency {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.code.hash(state);
        }
    }
    /// Input for creating a new move.
    pub struct CurrencyInput {
        /// A currency code. For example, `BHD`.
        pub code: String,
        /// The amount of decimal places the currency has. In the example of `BHD`, it would be 3.
        pub decimal_places: u8,
    }
    /// Represents money either taken out of or put into an account.
    ///
    /// A group of multiple moves can make up a [transaction](crate::entities::Transaction).
    #[derive(PartialEq, Debug, Clone)]
    pub struct Move<'book> {
        account: &'book Account,
        currency: &'book Currency,
        amount: i64,
    }
    impl<'book> Move<'book> {
        /// Creates a new move.
        ///
        /// ```
        /// use envelope_system::entities::*;
        /// let wallet = Account::new(AccountInput{});
        /// let thb = Currency::new(CurrencyInput{ code: "THB".into(), decimal_places: 2 });
        /// let move_ = Move::new(MoveInput{account: &wallet, currency: &thb, amount: 2000 });
        /// ```
        pub fn new(input: MoveInput<'book>) -> Self {
            let MoveInput {
                account,
                currency,
                amount,
            } = input;
            Self {
                account,
                currency,
                amount,
            }
        }
    }
    /// Input for creating a new move.
    pub struct MoveInput<'book> {
        /// Used to reference an [account](Account) in the [book](crate::book::Book).
        pub account: &'book Account,
        /// Used to reference a [currency](Currency) in the [book](crate::book::Book) that represents the currency moved.
        pub currency: &'book Currency,
        /// Specifies the amount moved, in minor unit of currency.
        pub amount: i64,
    }
    /// Represents an incomplete transaction that may yet be imbalanced.
    ///
    /// Entities of this type may be finalized into [transaction](Transaction)s using [TransactionDraft::finalize].
    ///
    /// ```
    /// use envelope_system::entities::*;
    /// let mut draft = TransactionDraft::new(TransactionDraftInput{});
    /// let atm = Account::new(AccountInput{});
    /// let thb = Currency::new(CurrencyInput{code: "THB".into(), decimal_places: 2});
    /// draft.add_move(Move::new(MoveInput{account: &atm, currency: &thb, amount: -1000 }));
    /// let wallet = Account::new(AccountInput{});
    /// draft.add_move(Move::new(MoveInput{account: &wallet, currency: &thb, amount: 1000 }));
    /// let transaction: Transaction = draft.finalize();
    /// ```
    pub struct TransactionDraft<'book> {
        moves: Vec<Move<'book>>,
    }

    #[test]
    fn move_new() {
        let wallet = Account::new(AccountInput {});
        let currency = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let amount = 2000;
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &currency,
            amount,
        });
        assert_eq!(move_.account, &wallet);
        assert_eq!(move_.currency, &currency);
        assert_eq!(move_.amount, 2000);
    }

    impl<'book> TransactionDraft<'book> {
        /// New transaction drafts start with no moves.
        /// Moves are to be added using [TransactionDraft::add_move].
        pub fn new(input: TransactionDraftInput) -> Self {
            Self { moves: Vec::new() }
        }
        /// Adds a move to the transaction draft.
        pub fn add_move(&mut self, move_: Move<'book>) {
            self.moves.push(move_);
        }
        /// Calculates the balances from the moves of the dransaction draft, one balance per currency.
        // TODO: the key should be `Currency`
        pub fn balances(&self) -> HashMap<&Currency, i64> {
            self.moves.iter().fold(HashMap::new(), |mut acc, curr| {
                let amount = curr.amount;
                let new_balance = acc
                    .get(curr.currency)
                    .map_or(amount, |balance| balance + amount);
                acc.insert(curr.currency, new_balance);
                acc
            })
        }
        /// Figures out whether all of the balances are zero.
        pub fn are_balanced(balances: &HashMap<&Currency, i64>) -> bool {
            balances.iter().all(|(_, balance)| *balance == 0)
        }
        /// If all of the balances are found to be zero, the draft will be finalized.
        ///
        /// ## Panics
        /// If any of the balances are not zero.
        pub fn finalize(self) -> Transaction<'book> {
            if Self::are_balanced(&self.balances()) {
                Transaction { moves: self.moves }
            } else {
                panic!("Transaction draft not balanced.");
            }
        }
    }
    /// Input for creating a new transaction draft.
    pub struct TransactionDraftInput {}
    /// A group of related [move](crate::entities::Move)s that all occur at some time.
    ///
    /// Transactions cannot be created directly.
    /// They start as [draft](crate::entities::TransactionDraft)s.
    #[test]
    fn transaction_draft_new() {
        let draft = TransactionDraft::new(TransactionDraftInput {});
        assert_eq!(draft.moves.len(), 0);
    }
    #[test]
    fn transaction_draft_add_move() {
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        let wallet = Account::new(AccountInput {});
        let currency = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &currency,
            amount: -1000,
        });
        let move_clone = move_.clone();
        draft.add_move(move_);
        assert_eq!(draft.moves.len(), 1);
        assert_eq!(draft.moves[0], move_clone);
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &currency,
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
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let wallet = Account::new(AccountInput {});
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &thb,
            amount: -1000,
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[&thb], -1000,);
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &thb,
            amount: 1000,
        });
        draft.add_move(move_);
        let balances = draft.balances();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[&thb], 0);
        let ils = Currency::new(CurrencyInput {
            code: "ILS".into(),
            decimal_places: 2,
        });
        let move_ = Move::new(MoveInput {
            account: &wallet,
            currency: &ils,
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
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let ils = Currency::new(CurrencyInput {
            code: "ILS".into(),
            decimal_places: 2,
        });
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
        let wallet = Account::new(AccountInput {});
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        draft.add_move(Move::new(MoveInput {
            account: &wallet,
            currency: &thb,
            amount: 10000,
        }));
        draft.finalize();
    }
    #[test]
    fn finalize_balanced_transaction_draft() {
        let draft = TransactionDraft::new(TransactionDraftInput {});
        let _transaction: Transaction = draft.finalize();
        let mut draft = TransactionDraft::new(TransactionDraftInput {});
        let wallet = Account::new(AccountInput {});
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        draft.add_move(Move::new(MoveInput {
            account: &wallet,
            currency: &thb,
            amount: 10000,
        }));
        draft.add_move(Move::new(MoveInput {
            account: &wallet,
            currency: &thb,
            amount: -10000,
        }));
        let _transaction: Transaction = draft.finalize();
    }
    pub struct Transaction<'book> {
        moves: Vec<Move<'book>>,
    }
    /// Represents an account. For example, a bank account or a wallet.
    ///
    /// ## Example:
    /// ```
    /// use envelope_system::entities::{Account, AccountInput};
    /// let wallet = Account::new(AccountInput{});
    /// ```
    #[derive(PartialEq, Debug)]
    pub struct Account {}
    /// Input for creating a new account.
    pub struct AccountInput {}
    impl Account {
        /// Creates a new account.
        pub fn new(input: AccountInput) -> Self {
            Self {}
        }
    }
}
/// Changes that can be applied to a [book](book::Book).
pub mod changes {
    use crate::book::Book;
    use crate::{entities, entities::Currency};
    /// Describes a change that can be applied to a [book](Book).
    ///
    /// A set of changes is provided.
    /// Custom changes are not supported; the fields of [book](Book) are private.
    pub trait Change {
        /// Applies a change to a book.
        fn apply(self, book: &mut Book);
    }
    /// Adds a [currency](Currency) to a [book](Book).
    ///
    /// ```
    /// use envelope_system::{ changes::*, book::Book, entities::* };
    /// let mut book = Book::new();
    /// let currency = Currency::new(CurrencyInput{ code: "THB".into(), decimal_places: 2 });
    /// let add_currency = AddCurrency::new(AddCurrencyInput{currency});
    /// book.apply(add_currency);
    /// ```
    pub struct AddCurrency {
        pub(crate) currency: Currency,
    }
    impl AddCurrency {
        /// Creates a new AddCurrency change.
        pub fn new(input: AddCurrencyInput) -> Self {
            Self {
                currency: input.currency,
            }
        }
    }
    /// Input for creating a new [AddCurrency](AddCurrency).
    pub struct AddCurrencyInput {
        pub currency: Currency,
    }
    impl Change for AddCurrency {
        /// Applies the change.
        ///
        /// ## Panics
        /// On application, if a currency with the same code already exists in the [book](Book).
        fn apply(self, book: &mut Book) {
            if book
                .currencies
                .values()
                .any(|currency| self.currency == *currency)
            {
                panic!("Currency code already exists.");
            } else {
                book.currencies.insert(self.currency);
            }
        }
    }
    #[test]
    #[should_panic(expected = "Currency code already exists.")]
    fn add_currency_code_exists() {
        use entities::*;
        let mut book = Book::new();
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let add_currency = AddCurrency::new(AddCurrencyInput { currency: thb });
        book.apply(add_currency);
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let add_currency = AddCurrency::new(AddCurrencyInput { currency: thb });
        book.apply(add_currency);
    }
    #[test]
    fn add_currency() {
        use entities::*;
        let mut book = Book::new();
        let thb = Currency::new(CurrencyInput {
            code: "THB".into(),
            decimal_places: 2,
        });
        let thb_clone = thb.clone();
        let add_currency = AddCurrency::new(AddCurrencyInput { currency: thb });
        book.apply(add_currency);
        assert_eq!(*book.currencies.iter().next().unwrap().1, thb_clone);
        let ils = Currency::new(CurrencyInput {
            code: "ILS".into(),
            decimal_places: 2,
        });
        let ils_clone = ils.clone();
        let add_currency = AddCurrency::new(AddCurrencyInput { currency: ils });
        book.apply(add_currency);
        assert_eq!(*book.currencies.iter().nth(1).unwrap().1, ils_clone);
    }
    /// Adds an [account](entities::Account) to a [book](Book).
    ///
    /// ```
    /// use envelope_system::{ changes::*, book::Book, entities::* };
    /// let mut book = Book::new();
    /// let account = Account::new(AccountInput{});
    /// let add_account = AddAccount::new(AddAccountInput{account});
    /// book.apply(add_account);
    /// ```
    pub struct AddAccount {
        pub(crate) account: entities::Account,
    }
    impl AddAccount {
        /// Creates a new AddAcccount change.
        pub fn new(input: AddAccountInput) -> Self {
            let AddAccountInput { account } = input;
            Self { account }
        }
    }
    /// Input for creating a new [AddAccount](AddAccount).
    pub struct AddAccountInput {
        pub account: entities::Account,
    }
    impl Change for AddAccount {
        /// Applies the change.
        fn apply(self, book: &mut Book) {
            book.accounts.insert(self.account);
        }
    }
    #[test]
    fn change_add_account() {
        use entities::*;
        let mut book = Book::new();
        let account = Account::new(AccountInput {});
        book.apply(AddAccount::new(AddAccountInput { account }));
        assert_eq!(book.accounts.len(), 1);
        assert_eq!(
            *book.accounts.iter().next().unwrap().1,
            Account::new(AccountInput {})
        )
    }
}
pub mod book {
    use crate::changes::Change;
    use crate::{entities, entities::Currency};
    use slotmap::{new_key_type, DenseSlotMap};
    new_key_type! {
        pub struct CurrencyKey;
        pub struct TransactionDraftKey;
        pub struct TransactionKey;
        pub struct AccountKey;
    }
    pub struct Book<'book> {
        pub(crate) currencies: DenseSlotMap<CurrencyKey, Currency>,
        pub(crate) transactions: DenseSlotMap<TransactionKey, entities::Transaction<'book>>,
        pub(crate) accounts: DenseSlotMap<AccountKey, entities::Account>,
    }
    impl<'book> Book<'book> {
        pub fn new() -> Self {
            Book {
                currencies: DenseSlotMap::with_key(),
                transactions: DenseSlotMap::with_key(),
                accounts: DenseSlotMap::with_key(),
            }
        }
        pub fn apply(&mut self, change: impl Change) {
            change.apply(self);
        }
    }
    #[test]
    fn book_new() {
        let book = Book::new();
        assert_eq!(book.currencies.len(), 0);
        assert_eq!(book.accounts.len(), 0);
        assert_eq!(book.transactions.len(), 0);
    }
}

// TODO: Nicer export paths
// TODO: Make change application panics docs more visible

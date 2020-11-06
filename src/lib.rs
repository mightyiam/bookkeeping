/// The various entities involved in accounting
///
/// Each entity is in its own submodule.
/// Each submodule includes:
/// - `Entity`: a struct that represents the entity.
/// - `Input`: a struct that serves as input for the `new` method of the `Entity` struct.
///
/// ## Example:
/// ```
/// use envelope_system::entities::*;
/// let wallet = Account::new(NewAccount{ name: String::from("Wallet") });
/// ```
pub mod entities {
    use crate::book::AccountKey;
    use rusty_money::Money;
    /// Represents money either taken out of or put into an account.
    ///
    /// A group of multiple moves can make up a [transaction](crate::entities::transaction).
    ///
    /// The purpose of the trailing underscore is to refrain from using the keyword [`move`](https://doc.rust-lang.org/std/keyword.move.html).
    pub struct Move {
        account_key: AccountKey,
        money: Money,
    }
    impl Move {
        pub fn new(input: NewMove) -> Self {
            let NewMove { account_key, money } = input;
            Self { account_key, money }
        }
    }
    pub struct NewMove {
        pub account_key: AccountKey,
        pub money: Money,
    }
    pub struct TransactionDraft {
        moves: Vec<Move>,
    }
    /// A group of related [move](crate::entities::move_)s that all occur at some time.
    ///
    /// Transactions cannot be created directly.
    /// They start as [draft](crate::entities::transaction_draft)s.
    pub struct Transaction {
        moves: Vec<Move>,
    }
    #[derive(PartialEq, Debug)]
    pub struct Account {
        name: String,
    }
    pub struct NewAccount {
        pub name: String,
    }
    impl Account {
        pub fn new(input: NewAccount) -> Self {
            Self { name: input.name }
        }
    }
}
mod changes {
    use crate::book::{Book, ChangeApplicationFailure};
    use crate::entities;
    use rusty_money::Currency;
    pub trait Change {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure>;
    }
    pub struct NewAddCurrency {
        pub currency: &'static Currency,
    }
    pub struct AddCurrency {
        pub(crate) currency: &'static Currency,
    }
    impl AddCurrency {
        pub fn new(payload: NewAddCurrency) -> Self {
            Self {
                currency: payload.currency,
            }
        }
    }
    impl Change for AddCurrency {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure> {
            if book.currencies.contains_key(self.currency.iso_alpha_code) {
                Err(ChangeApplicationFailure::CurrencyAlreadyExists(
                    self.currency.iso_alpha_code.to_string(),
                ))
            } else {
                book.currencies
                    .insert(self.currency.iso_alpha_code, self.currency);
                Ok(())
            }
        }
    }
    pub struct NewAddAccount {
        pub account: entities::Account,
    }
    pub struct AddAccount {
        pub(crate) account: entities::Account,
    }
    impl AddAccount {
        pub fn new(input: NewAddAccount) -> Self {
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
    use rusty_money::Currency;
    use slotmap::{new_key_type, DenseSlotMap};
    use std::collections::HashMap;
    new_key_type! {
        pub struct CurrencyKey;
        pub struct TransactionDraftKey;
        pub struct TransactionKey;
        pub struct AccountKey;
    }
    pub struct Book {
        pub(crate) currencies: HashMap<&'static str, &'static Currency>,
        pub(crate) transaction_drafts:
            DenseSlotMap<TransactionDraftKey, entities::TransactionDraft>,
        pub(crate) transactions: DenseSlotMap<TransactionKey, entities::Transaction>,
        pub(crate) accounts: DenseSlotMap<AccountKey, entities::Account>,
    }
    impl Book {
        pub fn new() -> Self {
            Book {
                currencies: HashMap::new(),
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
            use crate::entities;
            use rusty_money::{Currency, Iso};
            #[test]
            fn change_add_currency() {
                let mut book = Book::new();
                book.apply(changes::AddCurrency::new(changes::NewAddCurrency {
                    currency: Currency::get(Iso::THB),
                }));
                assert_eq!(book.currencies.len(), 1);
                assert_eq!(
                    *book.currencies.get("THB").unwrap(),
                    Currency::find_by_alpha_iso(String::from("THB")).unwrap(),
                );
            }
            #[test]
            fn change_add_account() {
                let mut book = Book::new();
                let account = entities::Account::new(entities::NewAccount {
                    name: String::from("Wallet"),
                });
                book.apply(changes::AddAccount::new(changes::NewAddAccount { account }));
                assert_eq!(book.accounts.len(), 1);
                assert_eq!(
                    *book.accounts.iter().next().unwrap().1,
                    entities::Account::new(entities::NewAccount {
                        name: String::from("Wallet"),
                    })
                )
            }
        }
    }
}

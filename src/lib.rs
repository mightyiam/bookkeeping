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
/// let wallet = Account::new(NewAccountArgs{ name: String::from("Wallet") });
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
        pub fn new(input: NewMoveArgs) -> Self {
            let NewMoveArgs { account_key, money } = input;
            Self { account_key, money }
        }
    }
    pub struct NewMoveArgs {
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
    pub struct NewAccountArgs {
        pub name: String,
    }
    impl Account {
        pub fn new(input: NewAccountArgs) -> Self {
            Self { name: input.name }
        }
    }
}
mod changes {
    use crate::book::{Book, ChangeApplicationFailure};
    pub trait BookChange {
        fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure>;
    }
    pub mod add_currency {
        use super::BookChange;
        use crate::book::{Book, ChangeApplicationFailure};
        use rusty_money::Currency;
        pub struct Input {
            pub currency: &'static Currency,
        }
        pub struct Change {
            pub(crate) currency: &'static Currency,
        }
        impl Change {
            pub fn new(input: Input) -> Self {
                Self {
                    currency: input.currency,
                }
            }
        }
        impl BookChange for Change {
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
    }
    pub mod add_account {
        use super::{BookChange, ChangeApplicationFailure};
        use crate::book::Book;
        use crate::entities;
        pub struct Input {
            pub account: entities::Account,
        }
        pub struct Change {
            pub(crate) account: entities::Account,
        }
        impl Change {
            pub fn new(input: Input) -> Self {
                Self {
                    account: input.account,
                }
            }
        }
        impl BookChange for Change {
            fn apply(self, book: &mut Book) -> Result<(), ChangeApplicationFailure> {
                book.accounts.insert(self.account);
                Ok(())
            }
        }
    }
}
pub mod book {
    use crate::changes::BookChange;
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
        pub fn apply(&mut self, change: impl BookChange) -> Result<(), ChangeApplicationFailure> {
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
                book.apply(changes::add_currency::Change::new(
                    changes::add_currency::Input {
                        currency: Currency::get(Iso::THB),
                    },
                ));
                assert_eq!(book.currencies.len(), 1);
                assert_eq!(
                    *book.currencies.get("THB").unwrap(),
                    Currency::find_by_alpha_iso(String::from("THB")).unwrap(),
                );
            }
            #[test]
            fn change_add_account() {
                let mut book = Book::new();
                let account = entities::Account::new(entities::NewAccountArgs {
                    name: String::from("Wallet"),
                });
                book.apply(changes::add_account::Change::new(
                    changes::add_account::Input { account },
                ));
                assert_eq!(book.accounts.len(), 1);
                assert_eq!(
                    *book.accounts.iter().next().unwrap().1,
                    entities::Account::new(entities::NewAccountArgs {
                        name: String::from("Wallet"),
                    })
                )
            }
        }
    }
}

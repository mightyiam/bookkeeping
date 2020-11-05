use rust_decimal::Decimal;
use rusty_money::{Currency, Iso, Money, MoneyError};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod entities {
    use crate::book::AccountKey;
    use rusty_money::Money;
    pub struct Move {
        account_key: AccountKey,
        money: Money,
    }
    pub struct TransactionDraft {
        moves: Vec<Move>,
    }
    pub struct Transaction {
        moves: Vec<Move>,
    }
    pub struct Account {
        name: String,
    }
}
mod changes {
    pub enum Change {
        Currency(currency::Change),
        Account(account::Change),
    }
    pub mod currency {
        pub enum Change {
            Add(add::Payload),
        }
        pub mod add {
            use rusty_money::{Currency, Iso};
            pub struct Input {
                pub code: Iso,
            }
            pub struct Payload {
                pub(crate) currency: &'static Currency,
            }
            impl Payload {
                pub fn new(input: Input) -> Self {
                    Self {
                        currency: Currency::get(input.code),
                    }
                }
            }
        }
    }
    pub mod account {
        pub enum Change {
            Add(add::Payload),
        }
        /*pub struct Change(Change_);

        impl Change {
            pub fn add(name: String) -> Change {
                Change(Change_::Add(name))
            }
        }*/
        mod add {
            pub struct Input {
                pub name: String,
            }
            pub struct Payload {
                name: String,
            }
            impl Payload {
                fn new(input: Input) -> Self {
                    Self { name: input.name }
                }
            }
        }
    }
}
pub mod book {
    use crate::changes;
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
        pub fn apply(&mut self, change: changes::Change) -> Result<(), ChangeApplicationFailure> {
            match change {
                changes::Change::Currency(change) => match change {
                    changes::currency::Change::Add(payload) => {
                        if self.currencies.contains_key(payload.currency.iso_alpha_code) {
                            Err(ChangeApplicationFailure::CurrencyAlreadyExists(
                                payload.currency.iso_alpha_code.to_string(),
                            ))
                        } else {
                            self.currencies
                                .insert(payload.currency.iso_alpha_code, payload.currency);
                            Ok(())
                        }
                    }
                },
                changes::Change::Account(change) => match change {
                    changes::account::Change::Add(payload) => {
                        let account = 
                        self.accounts.insert(account)
                    }
                },
            }
        }
    }
    pub enum ChangeApplicationFailure {
        CurrencyAlreadyExists(String),
    }
}
pub enum View {}
pub struct CurrencyView {
    code: String,
    decimal_places: u32,
}
pub type CurrenciesView = HashMap<String, Currency>;
#[cfg(test)]
mod tests {
    use crate::{book, changes, Currency, View};
    use rusty_money::Iso;
    use std::collections::HashMap;
    #[test]
    fn initial_state() {
        let book = book::Book::new();
        assert_eq!(book.currencies.len(), 0);
        assert_eq!(book.accounts.len(), 0);
        assert_eq!(book.transaction_drafts.len(), 0);
        assert_eq!(book.transactions.len(), 0);
    }
    #[test]
    fn struct_add_currency_change_new_errs_on_unknown_currency() {
        let unknown_currency = AddCurrencyChange::new(String::from("FOO"));
        if let Ok(_) = unknown_currency {
            panic!()
        }
    }
    #[test]
    fn change_add_currency() {
        let mut book = book::Book::new();
        book.apply(changes::Change::Currency(changes::currency::Change::Add(
            changes::currency::add::Payload::new(changes::currency::add::Input { currency: Currency::get(Iso::THB) }),
        )));
        assert_eq!(book.currencies.len(), 1);
        assert_eq!(
            *book.currencies.get("THB").unwrap(),
            Currency::find_by_alpha_iso(String::from("THB")).unwrap(),
        );
    }
    #[test]
    fn change_add_account() {
        let mut book = Book::new();
        book.apply(Change::AddAccount());
        assert_eq!()
    }
    fn inner_works() {
        let employer = Account::new("boss".to_string());
        let wallet = Account::new("wallet".to_string());
        let baht = Currency::new("THB".to_string(), 2);
        let _500_baht = baht.of(50000);
        let mut earn_500_baht = TransactionDraft::new();
        earn_500_baht.add_move(Move::new(&employer, baht.of(50000)));
        earn_500_baht.add_move(Move::new(&wallet, baht.of(50000)));
        book.add_transaction(&earn_500_baht.finalize());
        let wallet_balances_at_some_date: Vec<&Money> =
            buget.balances_at_date(&wallet, at_some_date);
        let wallet_balances_after_transaction =
            budget.balances_after_transaction(&wallet, &transaction);
    }
}

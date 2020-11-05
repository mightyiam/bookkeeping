mod entities {
    pub mod move_ {
        use crate::book::AccountKey;
        use rusty_money::Money;
        pub struct Entity {
            account_key: AccountKey,
            money: Money,
        }
    }
    pub mod transaction_draft {
        use super::move_::Entity as Move;
        pub struct Entity {
            moves: Vec<Move>,
        }
    }
    pub mod transaction {
        use super::move_::Entity as Move;
        pub struct Entity {
            moves: Vec<Move>,
        }
    }
    pub mod account {
        #[derive(PartialEq, Debug)]
        pub struct Entity {
            name: String,
        }
        pub struct Input {
            pub name: String,
        }
        impl Entity {
            pub fn new(input: Input) -> Self {
                Self { name: input.name }
            }
        }
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
            use rusty_money::Currency;
            pub struct Input {
                pub currency: &'static Currency,
            }
            pub struct Payload {
                pub(crate) currency: &'static Currency,
            }
            impl Payload {
                pub fn new(input: Input) -> Self {
                    Self {
                        currency: input.currency,
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
        pub mod add {
            use crate::entities;
            pub struct Input {
                pub account: entities::account::Entity,
            }
            pub struct Payload {
                pub(crate) account: entities::account::Entity,
            }
            impl Payload {
                pub fn new(input: Input) -> Self {
                    Self {
                        account: input.account,
                    }
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
            DenseSlotMap<TransactionDraftKey, entities::transaction_draft::Entity>,
        pub(crate) transactions: DenseSlotMap<TransactionKey, entities::transaction::Entity>,
        pub(crate) accounts: DenseSlotMap<AccountKey, entities::account::Entity>,
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
                        if self
                            .currencies
                            .contains_key(payload.currency.iso_alpha_code)
                        {
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
                        self.accounts.insert(payload.account);
                        Ok(())
                    }
                },
            }
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
                book.apply(changes::Change::Currency(changes::currency::Change::Add(
                    changes::currency::add::Payload::new(changes::currency::add::Input {
                        currency: Currency::get(Iso::THB),
                    }),
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
                let account = entities::account::Entity::new(entities::account::Input {
                    name: String::from("Wallet"),
                });
                book.apply(changes::Change::Account(changes::account::Change::Add(
                    changes::account::add::Payload::new(changes::account::add::Input { account }),
                )));
                assert_eq!(book.accounts.len(), 1);
                assert_eq!(
                    *book.accounts.iter().next().unwrap().1,
                    entities::account::Entity::new(entities::account::Input {
                        name: String::from("Wallet"),
                    })
                )
            }
        }
    }
}

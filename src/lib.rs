#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]
//#![deny(private_intra_doc_links)]
#![deny(missing_crate_level_docs)]
//#![deny(missing_doc_code_examples)]
//#![deny(private_doc_tests)]
#![deny(invalid_codeblock_attributes)]
#![doc(test(attr(deny(warnings))))]
#![feature(external_doc)]
#![doc(include = "../readme.md")]
#[macro_use]
mod macros;
mod account;
mod balance;
mod book;
mod move_;
mod sum;
mod transaction;
mod unit;
pub use account::Account;
pub use balance::Balance;
pub use book::{AccountKey, Book, Side, UnitKey};
pub use move_::Move;
pub use sum::Sum;
pub use transaction::Transaction;
pub use unit::Unit;

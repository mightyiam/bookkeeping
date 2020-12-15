#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]
//#![deny(private_intra_doc_links)]
#![deny(missing_crate_level_docs)]
#![deny(missing_doc_code_examples)]
//#![deny(private_doc_tests)]
#![deny(invalid_codeblock_attributes)]
#![doc(test(attr(deny(warnings))))]
//! ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github)
//!
//! This crate tries to model the very basics of the [bookkeeping](https://en.wikipedia.org/wiki/Bookkeeping) activity.
//! _it is a new rustacean's first open source crate_.
//!
//! ## The outline
//!
//! A book contains
//! - accounts,
//! - units (may represent currencies)
//! - and transactions, which in turn, contain moves.
//!
//! ## Features
//!
//! - Book balance guaranteed at compile time.
//! - Arbitrary metadata may be stored in books, accounts, units, transactions and moves.
//!
//! ## Defficiencies
//!
//! - No optimization of balance calculations.
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

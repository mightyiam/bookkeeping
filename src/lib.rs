#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
//#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]
//#![deny(private_intra_doc_links)]
//#![deny(missing_crate_level_docs)]
//#![deny(private_doc_tests)]
#![deny(invalid_codeblock_attributes)]
#![doc(test(attr(deny(warnings))))]
#![deny(clippy::all)]
//! ![GitHub Workflow Status (branch)][ci]
//!
//! This crate tries to model the very basics of the [bookkeeping][bookkeeping] activity.
//! _it is a new rustacean's first open source crate_.
//!
//! ## Outline
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
//! - No optimization of balance calculations.
//!
//! [ci]: https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github
//! [bookkeeping]: https://en.wikipedia.org/wiki/Bookkeeping
//!
//! ## Tutorial
//!
//! [Here][mod@tutorial].
//!
//! ## FAQ
//!
//! > This API can panic in a bunch of places. I don't like that. I don't feel safe.
//! > How about returning `Result`s, instead?
//!
//! `Result`s and errors are for when a function might fail despite all caution.
//! In this crate, panics would only occur on wrong usage.
//! Having this crate return `Result`s would complicate the API and — worse —
//! would give the impression that function calls could fail even when used correctly.
//! I'd like the user to be confident that with correct usage the API is safe.
#[macro_use]
mod test_utils;
macro_rules! tutorial {
    ($doc:expr) => {
        #[doc = $doc]
        pub mod tutorial {}
    };
}
tutorial!(include_str!("../tutorial.md"));
mod balance;
mod book;
mod move_;
mod sum;
mod transaction;
pub use balance::Balance;
pub use book::{AccountKey, Book, TransactionIndex};
pub use move_::{Move, Side};
pub use sum::Sum;
pub use transaction::{MoveIndex, Transaction};

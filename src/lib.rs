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
//! Minimal, safe and flexible [bookkeeping][bookkeeping] API
//!
//! ## Features
//!
//! - The book balance is guaranteed at compile time.
//! - Explicit ordering of transactions
//! - Strong support for multiple units (currencies)
//! - Use your own number types
//! - Arbitrary extra data
//! - A long [introduction][mod@introduction].
//!
//! ## Non-features
//!
//! Everything in this list is beyond the scope of this library:
//!
//! - Account types (asset/liability/income/expense/etc.)
//! - Reports
//!
//! ## Todo
//! - Cache balance calculations
//! - Serialization
//!
//! ## Introduction
//!
//! Read [the entire introduction][mod@introduction].
//!
//! ## A note on panics
//!
//! > "This API can panic in a bunch of places. I don't like that. I don't feel safe.
//! > How about returning `Result`s, instead?"
//!
//! `Result`s and errors are for when a function might fail despite all caution.
//! In this crate, panics would only occur on wrong usage.
//! Having this crate return `Result`s would complicate the API and — worse —
//! would give the impression that function calls could fail even when used correctly.
//! I'd like the user to be confident that with correct usage the API is safe.
//!
//! ## Get involved
//!
//! If you're using this crate, then please let me know—I'd be so happy!
//!
//! If you have a question, find an issue and/or would like to contribute,
//! please open an issue on the tracker or send me an email to
//! mightyiampresence@gmail.com.
//!
//! [ci]: https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github
//! [bookkeeping]: https://en.wikipedia.org/wiki/Bookkeeping
#[macro_use]
mod test_utils;
macro_rules! introduction {
    ($doc:expr) => {
        #[doc = $doc]
        pub mod introduction {}
    };
}
introduction!(include_str!("../introduction.md"));
mod balance;
mod book;
mod move_;
mod sum;
mod transaction;
pub use crate::{
    balance::Balance,
    book::{AccountKey, Book, TransactionIndex},
    move_::{Move, Side},
    sum::Sum,
    transaction::{MoveIndex, Transaction},
};

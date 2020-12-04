#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]
//#![deny(private_intra_doc_links)]
#![deny(missing_crate_level_docs)]
#![deny(missing_doc_code_examples)]
//#![deny(private_doc_tests)]
#![deny(invalid_codeblock_attributes)]
//! ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github)
//!
//! _This is a new rustacean's first open source crate.
//! It is made with the intention of serving as the go-to bookkeeping crate.
//! To get there, reviews from knowledgeable rustaceans seem necessary, so if you are one, then consider giving this crate your full attention for a while and leave your comments as a contribution to the community._
//!
//! This crate tries to model the very basics of the [bookkeeping](https://en.wikipedia.org/wiki/Bookkeeping) activity.
//!
//! It doesn't make assumptions regarding what metadata is attached to anything and allows you to provide your own types for that.
//! It also lets you determine how moves in an account are sorted, by providing your own sorting implementation that is based on your metadata.
//!
//! Imbalance is prevented as much as possible, first by the data structures and further entirely, by runtime.
//!
//! It attempts to provide an API that is nice to work with and to minimize user facing invariants.
//!
//! It is oblivious to the concept of a [currency](https://en.wikipedia.org/wiki/Currency).
//! Therefore, there are no decimal places, minor and major units.
//! There is only the concept of a [Unit], as in [unit of measurement](https://en.wikipedia.org/wiki/Unit_of_measurement), which the user may associate with some currency implementation via its metadata.
//!
//! The most obvious todo items are:
//! - Grouping of moves to represent transactions.
//! - Editing/removing items other than metadata.
//! - Caching of balance calculations.
#[macro_use]
mod macros;
mod account;
mod balance;
mod book;
mod move_;
mod sum;
mod unit;
pub use account::Account;
pub use balance::Balance;
pub use book::{AccountKey, Book, MoveKey, UnitKey};
pub use move_::Move;
pub use sum::Sum;
pub use unit::Unit;

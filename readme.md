![GitHub Workflow Status (branch)][ci]

This crate tries to model the very basics of the [bookkeeping][bookkeeping] activity.
_it is a new rustacean's first open source crate_.

## Outline

A book contains
- accounts,
- units (may represent currencies)
- and transactions, which in turn, contain moves.

## Features

- Book balance guaranteed at compile time.
- Arbitrary metadata may be stored in books, accounts, units, transactions and moves.

## Defficiencies

- No optimization of balance calculations.

[ci]: https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github
[bookkeeping]: https://en.wikipedia.org/wiki/Bookkeeping

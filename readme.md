![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github)

This crate tries to model the very basics of the [bookkeeping](https://en.wikipedia.org/wiki/Bookkeeping) activity.
_it is a new rustacean's first open source crate_.

## The outline

A book contains
- accounts,
- units (may represent currencies)
- and transactions, which in turn, contain moves.

## Features

- Book balance guaranteed at compile time.
- Arbitrary metadata may be stored in books, accounts, units, transactions and moves.

## Defficiencies

- No optimization of balance calculations.

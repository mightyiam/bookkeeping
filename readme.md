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

## Tutorial

### Moving money around and getting balances

```rust
// Yo, welcome to the `bookkeeping` crate.
use bookkeeping::*;
// You may be wondering why a crate tutorial starts with "Yo".
// Well, there's no good explanation for that. It is what it is.

// This crate aims to provide a neat API and amusing documentation.
// If you bear it through the tutorial, you should be able to start
// bookkeeping (that's a noun) in no-time.
//
// In case you didn't know, bookkeeping is about keeping record of money
// moving around. So, our goal in this tutorial is to teach you how to
// keep records of money moving around using this crate.

// The first thing you should know is that all records are stored in
// books. So, here's a new book:
let mut book = Book::<(), (), (), (), ()>::new(());
// "What are are all of these extra types?" — you must be wondering.
// Let me explain. You know what — I won't explain that right now.
// The important part is that we have a book.
// In that book, we can store accounts, units, transactions and moves.
// And doing all of that, is quite simple. So let's get to it.

// Let's start by adding an account for some income channel:
let income_key = book.new_account(());
// "What's that extra..." — we will get to that. Trust me.
// The important part is that we have an account.
// Actually, _the book_ has an account. What we own is an account key.
// We will later use this key to reference this account.

// It's nice that we have an account, so let's have another one!
let bank_key = book.new_account(());
// And now, that we have two accounts, we can move money around.
// Which is exciting. I know. But, actually, we can't do that yet.
// Because we don't have any units. So let's talk about units.

// Units may represent currencies. Or cryptocurrencies. Or units of
// distance or volume... But if we're honest, they will usually
// represent some form of money. Yet, it's not this crate's scope to
// make such decisions. As far as this crate is concerned — they're just
// units. We'll talk more about this later.
let usd_key = book.new_unit(());
// Look — we have dollars! US dollars (USD). And that `()` argument —
// thank you for being patient and not mentioning it.

// Now that we have two accounts and a unit, we can move money around.
// Which is exciting. I know. But, actually, we can't do that yet.
// Because, in this crate moving money around is represented by _moves_.
// So, we know that we need a move. But... moves are not directly inside
// a book — they live inside _transactions_. So we'll make a transaction
// to hold the move:
book.insert_transaction(0, ());

// That `0` argument is the index in which to insert the transaction
// into the book. You see, a book holds a single ordered collection of
// transactions. So... this created a new transaction and inserted it
// into the book at index `0`.

// So the book now has two accounts, one unit and one transaction.
// So, now we can move money around. Which is exciting. I know.
// But, actually, we can't do that yet. Cool motif, huh? We now need a
// _sum_. "A what—now?" you ask? A sum. Look:
let mut sum = Sum::new();
sum.set_amount_for_unit(2000, usd_key);
// We have created a sum and set the amount of a specific unit in it.
// "Wait — support for multiple units?" (that's you, asking).
// Yes. Sums support multiple units. Thank Joe for that. It's his idea.
// We'll get to using multiple units later. For now, this sum represents
// 2000 USD.

// So now the book contains two accounts, one unit and one transaction
// and also a sum that we own directly. So now we can move money around.
// Exciting, isn't it? And this is as far as this motif goes.
// Because now we really can move money around. Look:
book.insert_move(0, 0, income_key, bank_key, sum, ());
// What this did is created a new move and inserted it into the existing
// transaction that is at index `0`. We only have one transaction, so
// that's where it is. And the move was inserted at index `0` in the
// transaction. You see — moves in a transaction are orderd. So, it's
// kind of like this:
//
// - book
//    0. transaction
//       0. move
//
// The move is of 2000 dollars from the income account and to the bank
// account. What a miracle...

// As you may have guessed, you may add more accounts, units,
// transactions and moves. Here are three points to get in your mind.
// You may know best how to do that.
// - Accounts and units are referenced by keys.
// - Transactions and moves are referenced by indexes.
// - The index of a transaction is its index in the whole book.
// - The index of a move is its index in the transaction it's inside of.
// OK, those are actually four points.

// At this point (no pun intended), we would like to see the balance of
// the accounts. Well, I would — and I'm writing this tutorial, so:
let balance = book.account_balance_at_transaction(income_key, 0);
assert_eq!(
    balance.amounts().collect::<Vec<_>>(),
    vec![(usd_key, &-2000)] // negative amount
);
let balance = book.account_balance_at_transaction(bank_key, 0);
assert_eq!(
    balance.amounts().collect::<Vec<_>>(),
    vec![(usd_key, &2000)] // positive amount
);
// Cool?

// Let's move more money around, just to confirm our understanding:
let wallet_key = book.new_account(());
// This created a new account that represents a wallet.
book.insert_transaction(1, ());
// This created a new empty transaction and inserted it at index `1`.
let mut sum = Sum::new();
sum.set_amount_for_unit(100, usd_key);
book.insert_move(1, 0, bank_key, wallet_key, sum, ());
// Created and inserted a move of 100 USD from the bank account to the
// wallet account. Isn't this fun?

// Now, let's see some balances, using the index of this most recent
// transaction:
let balance = book.account_balance_at_transaction(income_key, 1);
assert_eq!(
    balance.amounts().collect::<Vec<_>>(),
    vec![(usd_key, &-2000)]
);
let balance = book.account_balance_at_transaction(bank_key, 1);
assert_eq!(
    balance.amounts().collect::<Vec<_>>(),
    vec![(usd_key, &1900)]
);
let balance = book.account_balance_at_transaction(wallet_key, 1);
assert_eq!(
    balance.amounts().collect::<Vec<_>>(),
    vec![(usd_key, &100)]
);

// Now, let's insert a new transaction between the two existing ones:
book.insert_transaction(1, ());
let mut sum = Sum::new();
sum.set_amount_for_unit(1000, usd_key);
book.insert_move(1, 0, income_key, bank_key, sum, ());
// And look at a running balance of the bank account:
let bank_running_balance: Vec<i128> = [0, 1, 2]
    .iter()
    .map(|transaction_index| {
        book.account_balance_at_transaction(
            bank_key,
            *transaction_index,
        )
        .unit_amount(usd_key)
        .unwrap()
        .clone()
    })
    .collect();
assert_eq!(bank_running_balance, vec![2000, 3000, 2900]);

// So far, we've learned a few methods that insert data into the book
// and one that calculates a balance. You may have noticed that in order
// to call [Book::account_balance_at_transaction], we need to have both
// a key of an existing account and an index of an existing transaction.
// So, how can we obtain these? This way for accounts and units:
let _accounts: Vec<(AccountKey, &Account<()>)> =
    book.accounts().collect();
let _units: Vec<(UnitKey, &Unit<()>)> = book.units().collect();
// Note that the order of the iterator returned from [Book::accounts] is
// undefined. And this way for transactions:
let _transactions: Vec<(usize, &Transaction<(), ()>)> =
    book.transactions().enumerate().collect();
```

### Metadata

```rust
use bookkeeping::*;
// It's probably time to explain all those `()` arguments that we've so
// far been patient regarding. This crate allows arbitrary data to be
// attached/added/included in the book itself and all records in it:
// accounts, units, moves and transactions.
//
// When creating a book, the _types_ of these metadata must be provided.
// So far, `()` has been provided as the metadata type for all records.
// Let's define some non-`()` metadata types:
struct AccountMetadata {
    id: u8,
    name: &'static str,
}
struct UnitMetadata {
    currency_code: &'static str,
    decimal_places: u8,
}
let mut book: Book<u8, AccountMetadata, UnitMetadata, (), &str> =
    Book::new(5);
// In order, the types of metadata that are defined in this example are:
//
// - For the book itself, just a `u8`. Perhaps in your system, books are
//   identified with merely an integer.
// - For each account, there's an `id` of a `u8` and a `name` of a
//   `&'static str`.
// - With units, we'd like to represent currencies.
// - For moves, we seem to not require metadata in this example.
// - Transactions have merely a `&str` that perhaps is used as a note.
//
// Now, let's see how these metadata types are used:
assert_eq!(book.metadata(), &5);
// Alright!
let wallet_key = book.new_account(AccountMetadata {
    id: 7,
    name: "Wallet",
});
let bank_key = book.new_account(AccountMetadata {
    id: 8,
    name: "Bank",
});
assert_eq!(&book.get_account(wallet_key).metadata().id, &7);
assert_eq!(&book.get_account(wallet_key).metadata().name, &"Wallet");
assert_eq!(&book.get_account(bank_key).metadata().id, &8);
assert_eq!(&book.get_account(bank_key).metadata().name, &"Bank");
// Cool!
let usd_key = book.new_unit(UnitMetadata {
    currency_code: "USD",
    decimal_places: 2,
});
assert_eq!(&book.get_unit(usd_key).metadata().currency_code, &"USD");
assert_eq!(book.get_unit(usd_key).metadata().decimal_places, 2);
// Sweet!
book.insert_transaction(0, "Withdrawal");
assert_eq!(
    book.transactions().next().unwrap().metadata(),
    &"Withdrawal"
);
// Rad!
book.insert_move(0, 0, bank_key, wallet_key, Sum::new(), ());
assert_eq!(
    book.transactions()
        .next()
        .unwrap()
        .moves()
        .next()
        .unwrap()
        .metadata(),
    &(),
);
// Dope!
```

[ci]: https://img.shields.io/github/workflow/status/mightyiam/bookkeeping/Rust/master?logo=github
[bookkeeping]: https://en.wikipedia.org/wiki/Bookkeeping

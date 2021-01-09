# Introduction

This library models bookkeeping in a way that is probably different than
other software. So when reading this document, keep an open mind in
general and especially regarding the terms involved, such as "account", "transaction", etc..

```rust
use bookkeeping::{
    AccountKey,
    Book,
    MoveIndex,
    Sum,
    TransactionIndex,
};

// The `Book` type is the entry point to this API.
// For maximum flexibility, `Book` is generic on a number of types.
// In real usage, these generics would likely be substituted with
// structs that you'd use throughout your system.
// For this document, a `MyBook` concrete alias will be created from the
// generic `Book`, using some trivial types for the generics.
// The generics are listed below and each will be explained later.

type MyBook = Book::<
    // `Unit`: the type used for units (currencies)
    &'static str,
    // `SumNumber`: the number type used in the `Sum` type
    u64,
    // `Meta`: the metadata type of a `Book`
    &'static str,
    // `Account`: the type of an `Account`
    &'static str,
    // `TransactionMeta`: the metadata type of a `Transaction`
    &'static str,
    // `MoveMeta`: the metadata type of a `Move`
    &'static str,
>;

// Let's create a new `Book` in which we will then record some personal
// financial transactions.
//
let mut book = MyBook::new("My personal finance");
// In the above call, `"My personal finance"` is of the `Book`s generic
// `Meta` type. 

// So there exists an empty book. Let's add some accounts to it.
// Some examples of what an account in this book may represent are:
//
// - My bank account
// - My cash wallet
// - My salary
// - Shopping
// - A debt I owe Charley
// 
// An account represents one of:
//
// - A storage of a physical/digital amount that I own
// - Credit I am owed or debt I owe
// - An income or expense channel of mine
//
// Notice that the invariant in all of the different kinds of accounts
// is "I". And that is because this particular book records my personal
// finances.

// In order to record events in the book, we must understand how
// financial events are stored in the book. Each financial event is
// stored as a `Move` inside of a `Transaction`.
//
// - A move represent a sum having moved from one account to another.
// - A transaction is an ordered sequence of moves, used to group  
//   moves together.
//
// A `Book` holds an ordered collection of `Transaction`s.
// Each transaction holds an ordered collection of `Move`s.
// Books start out with 0 transactions in them, and similarly,
// transactions start out with 0 moves in them.
//
// Arbitrary metadata may be stored in transactions and moves.
//
// It is clear that transactions must be ordered in some way. This
// library choses to assign the task of ordering transactions to the
// user. Two alternative approaches were considered. The first
// alternative approach is to include some date-time field in the
// `Transaction` type and use that to sort by. This alternative was
// discarded because it was determined that this library should not
// make a decision regarding a date type, forcing the user to use some
// particular date type over another. The second approach that was
// considered is to bound the `TransactionMeta` generic by the `Ord`
// trait and then to sort transactions by their metadata. This approach
// was discarded, because while `Ord` means "total order", that is not
// sufficient, due to the possibility of values equaling each other. For
// example, several instances of the same exact date. That may result
// in several transactions in an account having the same balance, or a
// different bug, depending on implementation.
//
// Therefore:
// - Transactions in a book are identified by `TransactionIndex`
// - Moves in a book are identified by both a `TransactionIndex` and a
//   `MoveIndex`.
//
// Let's say we have received a salary payment and would like to record
// this event in the book. First, there must be an account in the book
// representing my salary (an income channel) and another account
// representing my bank account:
let salary_key = book.insert_account("Salary");
let bank_account_key = book.insert_account("Bank account");
// The type of the `&'static str` arguments is the one provided for the
// `Book`s generic `Account` type. Even though the name of this generic
// does not end with `Meta`, it is similar to the `Book`s `Meta` and
// `TransactionMeta` generics in the sense that you may use whatever
// type you see fit.
// 
// Notice that by inserting new accounts, we have obtained keys. These
// keys will be used to refer to these accounts when adding moves.
//
// Now, we will insert a new transaction:
book.insert_transaction(
    // In this example, we know from the code that at this point in
    // execution, the book is empty. So the only index at which to
    // insert a new transaction is 0. In real usage, a transaction would
    // likely be inserted relative to existing transactions, based on a
    // fresh query of them. Querying for existing transactions is
    // documented later in this document.
    TransactionIndex(0),
    // The type of this argument is the one provided for the `Book`s
    // generic type `TransactionMeta`. As with the `Book`s `Meta`
    // generic, this may be any type you find useful.
    "January 2021 salary"
);
// Finally, insert a single `Move` into the transaction.
book.insert_move(
    // `transaction_index`
    // We know where the transaction is, because it had just been
    // inserted into the book.
    TransactionIndex(0),
    // `move_index`
    // Since this transaction is empty, the only index to insert a move
    // at is 0.
    MoveIndex(0),
    // `debit_account_key`
    // This argument represents the debit account. The debit account is
    // the account from which a sum is moved; the origin account.
    salary_key,
    // `credit_account_key`
    // This argument represents the credit account. The credit account
    // is the account to which a sum is moved; the destination account.
    // 
    // Due to the direction of a move being explicit at the type level,
    // the balance of a book is guaranteed at compile time. The balance
    // of a book is a property where the sum of all account balances is
    // 0. In other words, all amounts are accounted for. No amount came
    // from thin air and no amount disappeared into thin air. Each move
    // has an origin account and a destination account.
//
    bank_account_key,
    // `sum`
    // The sum of the move, returned from a block:
    {
        // The `Sum` type in this library stores any number of unit
        // amounts. It is a mapping between units and amounts. Units may
        // represent currencies or cryptocurrencies or whatever else you
        // may be "bookkeeping". Therefore the more general term "unit".
        // We start with an empty sum and then we set its unit amounts.
        let mut sum = Sum::default();
        sum.set_amount_for_unit(
            // The type of this number that is stored in `Sum`s is the
            // `Book`s generic `SumNumber`, so you may use whatever
            // number type you find appropriate. Since the direction of
            // a move is explicitly defined by the `debit_account_Key`
            // and `credit_account_key` arguments, a negative value may
            // not be necessary here, so a number type that excludes
            // negative values may be used. For simplicity we use `u64`.
            4115,
            // The type of this argument is the `Book`s generic `Unit`
            // type. Feel free to use whatever type you like, as long as
            // it implements `Ord + Clone`.
            "USD"
        );
        // In this example, the sum of this move contains just one unit
        // amount; 4115 USD. Yet, this library supports the unusual case
        // of a sum (and therefore a move) containing multiple unit
        // amounts. Therefore a single move may serve to record that 100
        // USD and 3,000 THB had been moved together.
        sum
    },
    // `metadata`
    // The type of this value is the `Book`s `MoveMeta` generic. You are
    // free to provide any type you like. In this case, even though the
    // type is a `&'static str`, we use an empty string, because we have
    // nothing to say about this particular move. The transaction's
    // metadata already describes it well enough.
    "",
);
// Now that a move is recorded in the book, we can move on to the
// subject of calculating balances. A balance query is asking the
// question "how much is in account A at transaction index N?". Since
// the only move in the book is of 4115 USD from the salary account to
// the bank account, we know what balances to expect at those accounts:
// At transaction index 0 we expect the salary account to have -4115 USD
// and the bank account to have 4115 USD. So let's obtain these balances
// and assert these expectations. First, we will obtain the balance of
// the salary account.
//
// The generic type `BalanceNumber` of the following method, allows (and
// requires) you to provide a number type that will be used for the
// amounts in the returned `Balance`. Also, its implementations of `Sub`
// and `Add` will be used internally in calculating the balance. We will
// use `i128` here.
let salary_balance = book.account_balance_at_transaction::<i128>(
    // `account_key`
    // In this example, we know that this account key exists in the
    // book. In real usage, this account key would probably originate
    // from a fresh query to the book. Such queries are documented later
    // in this document.
    salary_key,
    // `transaction_index`
    // Similarly, in real usage, this index would probably originate
    // from a fresh query of the existing transactions in the book.
    TransactionIndex(0),
);
// Assert that the USD balance of the salary account is -4115:
assert_eq!(
    *salary_balance.unit_amount("USD").unwrap(),
    // Negative
    -4115,
);
// Obtain the balance of the bank account:
let bank_account_balance = book.account_balance_at_transaction::<i128>(
    bank_account_key,
    TransactionIndex(0),
);
// Assert that the USD balance of the bank acount is 4115:
assert_eq!(
    *bank_account_balance.unit_amount("USD").unwrap(),
    // Positive
    4115,
);
// Question: how can I get the _current_ balance of an account?
// Answer: query for the balance of that account at the very last
// transaction in the book.
//
// Question: how can I get the initial balance of an account, before
// any moves?
// Answer: there's no way to do that. If, in your usage, an initial
// balance is necessary, you'd probably make an initial move for each
// new account, upon its creation.

// Here are the different ways a book can be queried:
//
// Accounts:
let accounts: Vec<(AccountKey, &&str)> = book.accounts().collect();
// Let's assert that our accounts are in it:
assert!(accounts.contains(&(salary_key, &"Salary")));
assert!(accounts.contains(&(bank_account_key, &"Bank account")));
//
// An iterator of transactions may be obtained this way:
let mut transactions_iter = book.transactions();
let (_, salary_transaction) = transactions_iter.next().unwrap();
// Let's assert on the metadata of this transaction:
assert_eq!(*salary_transaction.metadata(), "January 2021 salary");
//
// An iterator of moves in a transaction may be obtained this way:
let (_, salary_move) = salary_transaction.moves().next().unwrap();
// Let's assert on the sum of the move:
assert_eq!(*salary_move.sum().unit_amount(&"USD").unwrap(), 4115);
```

In addition to this document, please read the [crate-level documentation][crate].

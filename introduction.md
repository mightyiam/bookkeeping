## Table of contents

- [Table of contents](#table-of-contents)
- [Terms](#terms)
  - [Unit](#unit)
  - [Sum](#sum)
  - [Book](#book)
  - [Account](#account)
  - [Move](#move)
  - [Transaction](#transaction)
  - [Balance](#balance)
- [Generics](#generics)
- [Why transactions and moves are explicitly ordered](#why-transactions-and-moves-are-explicitly-ordered)
- [Usage example](#usage-example)
- [Reference table of expectations](#reference-table-of-expectations)

This introduction will explain how this library models the [bookkeeping][bookkeeping] domain and how to use this library to perform bookkeeping.
This library models bookkeeping in a way that is probably different than other software.
So when reading this document, keep an open mind in general and especially regarding the terms involved, because they may be different than what you may already be familiar with.

## Terms

This library models bookkeeping using the following terms:

- Unit
- Sum
- Account
- Move
- Transaction
- Balance

### Unit

While currency may be the most common kind of unit in bookkeeping, this library could be used to "bookkeep" amounts of any number of units. Therefore, instead of using the term _currency_, this library uses the term _unit_.

### Sum

A _sum_ represents value — as in amounts of units. Possibly multiple amounts of different units.
For example, _$5_ is a sum. _$20 and €15_ is also a sum.

### Book

A storage device for financial records of a particular entity.
For example, a personal bookkeeping book or an organization's bookkeeping book.

### Account

What an account represents varies and is up to you.
Some kinds of accounts include:

- _Assets_ such as a bank account or a wallet
- _Income/expense channels_ such as "Salary" and "Shopping"
- _Debt/credit_ such as a loan

### Move

A _move_ represent the moving of a [sum](#sum) from one account to another.
A verbal representaion of a move could be:

> $1,000 and €900 have moved from my _wallet_ account into my _bank_ account.

In a move, one account is the debit account and the other account is the credit account.
In other words, origin account and destination account, respectively.
The explicit direction of moves is one of the properties allowing the balance of the book to be guaranteed at compile time.

The balance of a book is a property where the sum of all account balances is 0.
In other words, all amounts are accounted for.
No amount came from thin air and no amount disappeared into thin air.
Each move has a debit (origin) account and a credit (destination) account.

### Transaction

A book stores a single ordered collection of _transactions_.
Each transaction stores an ordered collection of _moves_.

Books start out with no transactions in them, and similarly,
transactions start out with no moves in them.

The existence of transactions is to provide the ability to group moves together.
Most financial events may be represented in one move, such as paying for a meal at a restaurant.
Yet, many finantial events may be represented in multiple moves, such as loan repayment, where part of it pays for the principal and the other part for interest.

### Balance

A balance can be defined as answering the following

> _How much is_ in account `x` _at_ transaction `i`?

Similar to a _sum_ a balance is a mapping from _units_ to amounts.

## Use your own unit and number types

- The type used to represent units is generic 
- The number types for both sums and balances are generic.
For balance calculations, the sum number type must be convertible into the balance number type.

## Extra data

- Arbitrary extra data may be stored in accounts, transactions and moves.

## Why transactions and moves are explicitly ordered

Assuming that transactions are ordered chronologically, obtaining the _latest_ balance of account `x` requires summation of _all_ the transactions that affect account `x`.
Yet, obtaining the balance of account `x` _at transaction `i`_ requires summation of the transactions that affect account `x` _prior to and including transaction `i`_. 
So it is clear that transactions _must be ordered_ in some way—yet in what way?
Obviously, a chronologic order makes sense, yet this library assigns the task of ordering transactions to the
user.
Two alternative approaches were considered.
The first alternative approach is that some date-time field is included in the transaction and is used to sort transactions by.
This alternative was discarded because it was determined that this library should not make a decision regarding a date type, forcing the user to use some particular date type over another.
The second alternative that was considered is binding the transaction extra data generic by the [std::cmp::Ord] trait and then sorting transactions by their extra data.
This approach was discarded, because while `Ord` means "total order", that is not sufficient, due to [the possibility of values equaling each other][ord-equal].
That may result
in several transactions in an account having the same balance, or a
different bug, depending on implementation.

Note: While the property of moves in a transaction _being ordered_ did not seem necessary, an un-ordered design did not seem to result in reduced implementation complexity.

## Usage example

Here is a list of financial events that we will record into memory in this example:

1. When I started bookkeeping my personal finances, I had $8,000 and €1,000 in the bank and $200 in my wallet.
2. I was paid a €6,000 salary, out of which €100 went to health insurance.
3. I have converted €5,000 into $6,000 for a fee of €10 at the bank.
4. I made a bank tranfer of $1,200 rent payment that includes my flatmate Charley's part—which they now owe me.
5. Charley paid me back in cash.

```rust
// In order to record our personal finances we'll need a book.

use bookkeeping::Book;

// The `Book` type is the entry point to this library's API.
// For tight integration into your system, `Book` is generic over a few
// type parameters. We will create a concrete type alias for `Book`
// where primitive concrete types are provided for the generics.

type MyBook = Book::<char, u64, &'static str, &'static str, ()>;

// The generic type parameters of the book, in order:

// 1. `Unit`: represents units in sums and balances. We will use `char`,
//    such as `'$'` and `'€'`.
// 2. `SumNumber`: represents the number type in sums. Since the
//    direction of a move is explicit, a number type that excludes
//    negative values may be used. We will use `u64`.
// 3. `AccountExtra`: Arbitrary data attached to accounts. We will use
//    static lifetime string slices, such as `"Bank"` and `"Income"`.
// 4. `TransactionExtra`: arbitrary data attached to transactions. We
//    will use static lifetime string slices, such as `"Rent payment"`.
// 5. `MoveExtra`: arbitrary data attached to moves. For simplicity, we
//    won't be using this generic, setting it to `()`.

// In real usage, more advanced types will probably be used.

// And here is our empty book value:

let mut book = MyBook::default();

// Finantial event 1:

// > When I started bookkeeping my personal finances, I had $8,000 and
// > €1,000 in the bank and $200 in my wallet.

// We will create these accounts and insert them into the book:

use bookkeeping::AccountKey;
let bank: AccountKey = book.insert_account("Bank");
let wallet: AccountKey = book.insert_account("Wallet");

// `"Bank"` and `"Wallet"` are of the `Book`'s generic `AccountExtra` type.

// Notice that by inserting new accounts, we have obtained keys. These
// keys will be used to refer to these accounts when adding moves.

// For these new accounts to have initial balances, some other account,
// where these amounts originated from, must exist. For simplicity,
// let's have a single account that represents all income:

let income: AccountKey = book.insert_account("Income");

// We will use a single transaction to record initial balances for the
// wallet and the bank accounts. In this transaction we will insert a
// move from the income account to the bank account and another move,
// from the income account to the wallet account.

use bookkeeping::TransactionIndex;

book.insert_transaction(TransactionIndex(0), "Initial balances");

// The first argument is the index at which to insert the transaction.
// Since there are no transactions in the book, this is certainly 0.
// An out-of-bounds value would result in a panic.

// The second argument is of the `Book`s generic `TransactionExtra` type.

// Here is the move from the income account to the bank account:

use bookkeeping::{ MoveIndex, Sum };

book.insert_move(
    TransactionIndex(0),
    MoveIndex(0),
    income,
    bank,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(8000, '$');
        sum.set_amount_for_unit(1000, '€');
        sum
    },
    (),
);

// The first argument specifies the index of the transaction this move
// will be inserted into. An out-of-bounds value would result in a panic.

// The second argument specifies the index at which to insert this move
// into the transaction. The only possible index is 0 because the
// transaction has no moves. An out-of-bounds value would result in a
// panic. 

// The third and fourth arguments are the keys for the debit and credit
// accounts, respectively. Providing a key for which no account in the
// book exists would result in a panic.

// The fifth argument is the sum of the move. The numbers of the sum are
// of the `Book`s generic `SumNumber` type and the `'$'` and `'€'`
// values are of the `Book`s generic `Unit` type.

// The type of the sixth argument is the `Book`s generic `MoveExtra`,
// which we opted out of using.

// And here is the move from the income account to the wallet account:

book.insert_move(
    TransactionIndex(0),
    MoveIndex(1),
    income,
    wallet,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(200, '$');
        sum
    },
    (),
);

// Finantial event 2:

// > I was paid a €6,000 salary, out of which €100 went to health
// > insurance.

book.insert_transaction(TransactionIndex(1), "Salary");

book.insert_move(
    TransactionIndex(1),
    MoveIndex(0),
    income,
    bank,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(5900, '€');
        sum
    },
    (),
);

let expenses: AccountKey = book.insert_account("Expenses");

book.insert_move(
    TransactionIndex(1),
    MoveIndex(1),
    income,
    expenses,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(100, '€');
        sum
    },
    (),
);

// Finantial event 3:

// > I have converted €5,000 into $6,000 for a fee of €10 at the bank.

book.insert_transaction(TransactionIndex(2), "Conversion");

book.insert_move(
    TransactionIndex(2),
    MoveIndex(0),
    bank,
    expenses,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(5000, '€');
        sum
    },
    (),
);

book.insert_move(
    TransactionIndex(2),
    MoveIndex(1),
    bank,
    expenses,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(10, '€');
        sum
    },
    (),
);

book.insert_move(
    TransactionIndex(2),
    MoveIndex(2),
    income,
    bank,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(6000, '$');
        sum
    },
    (),
);

// Finantial event 4:

// > I made a bank tranfer of $1,200 rent payment that includes my
// > flatmate Charley's part—which they now owe me.

book.insert_transaction(TransactionIndex(3), "Everyone's rent");

book.insert_move(
    TransactionIndex(3),
    MoveIndex(0),
    bank,
    expenses,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(600, '$');
        sum
    },
    (),
);

let charley: AccountKey = book.insert_account("Charley");

book.insert_move(
    TransactionIndex(3),
    MoveIndex(1),
    bank,
    charley,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(600, '$');
        sum
    },
    (),
);

// Finantial event 5:

// > Charley paid me back in cash.

book.insert_transaction(TransactionIndex(4), "Charley paid me back");

book.insert_move(
    TransactionIndex(4),
    MoveIndex(0),
    charley,
    wallet,
    {
        let mut sum = Sum::default();
        sum.set_amount_for_unit(600, '$');
        sum
    },
    (),
);

// We are now supposed to be familiar with the principles of this
// library's bookkeeping model and the parts of its API that facilitate
// insertion of records into memory. The next topic is querying. To help
// with querying, we should have some expectations regarding the results
// of any queries. For this purpose, a table is provided immediately
// following this code block. It is intended to be reviewed at this
// point.

// Assert that some accounts exist in the book. For this we will use
// the `Book::accounts` method, which returns an
// `impl Iterator<Item=(AccountKey, &AccountExtra)>` that iterates in
// _undefined_ order.

let actual_accounts: Vec<(AccountKey, &&str)> = book
    .accounts()
    .collect();

assert!(actual_accounts.contains(&(bank, &"Bank")));
assert!(actual_accounts.contains(&(income, &"Income")));

// Assert some balances.

use bookkeeping::Balance;

// The bank account balance at transaction index 0 includes 8,000 USD.

let balance: Balance<char, i128> = book
    .account_balance_at_transaction(bank, TransactionIndex(0));

assert_eq!(*balance.unit_amount('$').unwrap(), 8000);

// The expenses account balance at transaction index 2 has 5,110 EUR.

let balance: Balance<char, i128> = book
    .account_balance_at_transaction(expenses, TransactionIndex(2));

assert_eq!(*balance.unit_amount('€').unwrap(), 5110);

// Assert some transaction extra data.

use bookkeeping::Transaction;

let (_, transaction): (_, &Transaction<char, u64, &str, ()>) = book
    .transactions()
    .nth(2)
    .unwrap();

assert_eq!(*transaction.extra(), "Conversion",);

// Assert some move properties

let (_, move_): (_, &Move<char, u64, ()>) = transaction.moves().nth(2).unwrap();

use bookkeeping::{ Move, Side::{ Credit, Debit } };

assert_eq!(move_.side_key(Debit), income);
assert_eq!(move_.side_key(Credit), bank);
assert_eq!(*move_.sum().unit_amount(&'$').unwrap(), 6000);
```

## Reference table of expectations

This table shows the accounts we expect to exist in the book and for each account the transactions that affect it and how they affect it, exactly.

<style>
td:nth-child(4) { text-align: right; }
</style>
<table style="font-size: 0.85em">
    <thead>
        <tr><th rowspan=2>Transaction</th><th colspan=2>Affecting moves</th><th rowspan=2>Balance</th></tr>
        <tr><th>Other side</th><th>Affect</th></tr>
    </thead>
    <tbody>
        <tr><th colspan=5>Income account</th></tr>
        <tr><td rowspan=2>0: Initial balances</td><td>Bank</td><td>-$8,000, -€1,000</td><td rowspan=2>-$8,200<br>-€1,000</td></tr>
        <tr><td>Wallet</td><td>$200</td>
        <tr><td rowspan=2>1: Salary</td><td>Bank</td><td>-€5,900</td><td rowspan=2>-$8,200<br>-€7,000</td></tr>
        <tr><td>Expenses</td><td>-€100</td></tr>
    </tbody>
    <tbody>
        <tr><th colspan=5>Bank account</th></tr>
        <tr><td>0: Initial balances</td><td>Income</td><td>$8,000, €1,000</td><td>$8,000<br>€1,000</td></tr>
        <tr><td>1: Salary</td><td>Income</td><td>€5,900</td><td>$8,000<br>€6,900</td></tr>
        <tr><td rowspan=3>2: Conversion</td><td>Expenses</td><td>-€5,000</td><td rowspan=3>$14,000<br>€1,890</td></tr>
        <tr><td>Expenses</td><td>-€10</td></tr>
        <tr><td>Income</td><td>+$6,000</td></tr>
        <tr><td rowspan=2>3: Everyone's rent</td><td>Rent</td><td>-$600</td><td rowspan=2>$12,800<br>€1,900</td></tr>
        <tr><td>Charley</td><td>-$600</td></tr>
    </tbody>
    <tbody>
        <tr><th colspan=5>Wallet account</th></tr>
        <tr><td>0: Initial balances</td><td>Income</td><td>$200</td><td>$200</td></tr>
        <tr><td>4: Charley paid me back</td><td>Charley</td><td>$600</td><td>$800</td></tr>
    </tbody>
    <tbody>
        <tr><th colspan=5>Expenses account</th></tr>
        <tr><td>1: Salary</td><td>Income</td><td>€100</td><td>€100</td></tr>
        <tr><td rowspan=2>2: Conversion</td><td>Bank</td><td>€5000</td><td rowspan=2>€5110</td></tr>
        <tr><td>Bank</td><td>€10</td></tr>
        <tr><td>3: Everyone's rent</td><td>Bank</td><td>$600</td><td>$600<br>€5110</td></tr>
    </tbody>
    <tbody>
        <tr><th colspan=5>Charley account</th></tr>
        <tr><td>3: Everyone's rent</td><td>Bank</td><td>$600</td><td>$600</td></tr>
        <tr><td>4: Charley paid me back</td><td>Wallet</td><td>-$600</td><td>$0</td></tr>
    </tbody>
</table>

In addition to this document, please read the [crate-level documentation][crate].

[bookkeeping]: https://en.wikipedia.org/wiki/Bookkeeping
[ord-equal]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html

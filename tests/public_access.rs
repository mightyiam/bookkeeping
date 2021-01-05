#![allow(path_statements)]
use bookkeeping::*;
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct TestUnit;
#[test]
fn account() {
    Account::<()>::metadata;
}
#[test]
fn balance() {
    type TestBalance = Balance<TestUnit, i128>;
    TestBalance::amounts;
    TestBalance::unit_amount;
}
#[test]
fn book() {
    type TestBook = bookkeeping::Book<TestUnit, u64, (), (), (), ()>;
    TestBook::new;
    TestBook::metadata;
    TestBook::set_book_metadata;
    TestBook::new_account;
    TestBook::insert_transaction;
    TestBook::insert_move;
    TestBook::get_account;
    TestBook::accounts;
    TestBook::transactions;
    TestBook::set_account_metadata;
    TestBook::set_transaction_metadata;
    TestBook::set_move_metadata;
    TestBook::account_balance_at_transaction::<i128>;
    TestBook::remove_move;
    TestBook::set_move_sum;
    TestBook::set_move_side;
}
#[test]
fn move_() {
    type TestMove = Move<TestUnit, u64, ()>;
    TestMove::debit_account_key;
    TestMove::credit_account_key;
    TestMove::sum;
    TestMove::metadata;
}
#[test]
fn sum() {
    type TestSum = Sum<TestUnit, u64>;
    TestSum::new;
    TestSum::set_amount_for_unit;
    TestSum::amounts;
}
#[test]
fn transaction() {
    type TestTransaction = Transaction<TestUnit, u64, (), ()>;
    TestTransaction::moves;
    TestTransaction::metadata;
}

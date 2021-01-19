#![allow(path_statements)]
use bookkeeping::*;
#[test]
fn balance() {
    type TestBalance = Balance<(), ()>;
    TestBalance::amounts;
    TestBalance::unit_amount;
}
#[test]
fn book() {
    type TestBook = bookkeeping::Book<(), u8, (), (), ()>;
    TestBook::default;
    TestBook::insert_account;
    TestBook::insert_transaction;
    TestBook::insert_move;
    TestBook::get_account;
    TestBook::accounts;
    TestBook::transactions;
    TestBook::set_account;
    TestBook::set_transaction_extra;
    TestBook::set_move_extra;
    TestBook::account_balance_at_transaction::<i16>;
    TestBook::remove_move;
    TestBook::set_move_sum;
    TestBook::set_move_side;
}
#[test]
fn move_() {
    type TestMove = Move<(), (), ()>;
    TestMove::side_key;
    TestMove::sum;
    TestMove::extra;
}
#[test]
fn sum() {
    type TestSum = Sum<(), u64>;
    TestSum::default;
    TestSum::set_amount_for_unit;
    TestSum::amounts;
}
#[test]
fn transaction() {
    type TestTransaction = Transaction<(), (), (), ()>;
    TestTransaction::moves;
    TestTransaction::extra;
}

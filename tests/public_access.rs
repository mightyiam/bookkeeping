#![allow(path_statements)]
use bookkeeping::*;
#[test]
fn account() {
    Account::<()>::metadata;
}
#[test]
fn balance() {
    Balance::amounts;
    Balance::unit_amount;
}
#[test]
fn book() {
    type TestBook = bookkeeping::Book<(), (), (), (), ()>;
    TestBook::new;
    TestBook::metadata;
    TestBook::set_book_metadata;
    TestBook::new_account;
    TestBook::new_unit;
    TestBook::insert_transaction;
    TestBook::insert_move;
    TestBook::get_account;
    TestBook::get_unit;
    TestBook::accounts;
    TestBook::units;
    TestBook::transactions;
    TestBook::set_account_metadata;
    TestBook::set_unit_metadata;
    TestBook::set_transaction_metadata;
    TestBook::set_move_metadata;
    TestBook::account_balance_at_transaction;
    TestBook::remove_move;
    TestBook::set_move_sum;
    TestBook::set_move_side;
}
#[test]
fn move_() {
    type TestMove = Move<()>;
    TestMove::debit_account_key;
    TestMove::credit_account_key;
    TestMove::sum;
    TestMove::metadata;
}
#[test]
fn sum() {
    Sum::new;
    Sum::set_amount_for_unit;
    Sum::amounts;
}
#[test]
fn transaction() {
    type TestTransaction = Transaction<(), ()>;
    TestTransaction::moves;
    TestTransaction::metadata;
}
#[test]
fn unit() {
    Unit::<()>::metadata;
}

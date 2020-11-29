#![feature(bindings_after_at)]
use chrono::Duration;
use std::error::Error;

use cool_asserts::assert_matches;
use envelope_system::book::*;

#[test]
fn new_book_has_no_accounts() {
    let book = Book::new();
    assert_eq!(book.accounts().count(), 0);
}

#[test]
fn look_up_nonexisting_account() {
    let book = Book::new();
    let acc = "acc";
    let ref acc_id = acc.into();
    assert_matches!(book.lookup_account(acc_id), Err(ref err @ LookupAccountError::DoesNotExist(ref id)) => {
        assert_eq!(err.to_string(), format!("account \"{}\" doesn't exist", acc));
        assert_eq!(id, acc_id);
    });
}

#[test]
fn transfer_from_nonexisting_accountq() {
    let ref thb = THB();
    let from = "from";
    assert_matches!(
        Book::new().transfer(&from.into(), &from.into(), thb.of_major(2)),
        Err(ref err) => {
            let inner = format!("account \"{}\" doesn't exist", from);
            assert_eq!(err.to_string(), format!("cannot transfer from account, {}", inner));
            assert_matches!(err.source(), Some(ref err) => {
                assert_eq!(err.to_string(), inner);
            });
        }
    );
}

#[test]
fn transfer_to_nonexisting_account() {
    let mut book = Book::new();
    let ref from = book.create_account("from").unwrap();
    let to = "to";
    let ref thb = THB();
    assert_matches!(
        book.transfer(from, &to.into(), thb.of_major(5)),
        Err(ref err) => {
            let inner = format!("account \"{}\" doesn't exist", to);
            assert_eq!(err.to_string(), format!("cannot transfer to account, {}", inner));
            assert_matches!(err.source(), Some(ref err) => {
                assert_eq!(err.to_string(), inner);
            });
        }
    );
}

#[test]
fn adding_one_account() {
    let mut book = Book::new();
    let wallet = &book.create_account("wallet").unwrap();
    assert_eq!(book.accounts().count(), 1);
    assert_matches!(book.accounts().next(), Some(account) => {
        assert_matches!(book.lookup_account(wallet), Ok(wallet) => {
            assert_eq!(account, wallet)
        })
    });
}

#[test]
fn adding_existing_account() {
    let mut book = Book::new();
    assert_matches!(book.create_account("acc"), Ok(acc) => {
        assert_matches!(
            book.create_account(acc.as_str()),
            Err(ref err @ CreateAccountError::AlreadyExists(ref id)) => {
                assert_eq!(*id, acc);
                assert_eq!(err.to_string(), format!("account \"{}\" already exists", acc));
            });
    });
}

#[test]
fn balance_of_nonexisting_account() {
    let acc = "acc";
    assert_matches!(
        Book::new().balance(&acc.into()),
        Err(ref err) => {
            assert_eq!(err.to_string(), format!("account \"{}\" doesn't exist", acc));
        }
    );
}

#[test]
fn balance_of_new_account() {
    let mut book = Book::new();
    let ref acc = book.create_account("acc").unwrap();
    assert_matches!(book.balance(acc), Ok(money) => {
        assert_eq!(money, Money::none());
    });
}

#[test]
fn transfer_own_account() {
    let mut book = Book::new();
    let ref acc = book.create_account("acc").unwrap();
    let ref thb = THB();
    book.transfer(acc, acc, thb.of_major(5)).unwrap();
    assert_matches!(book.balance(acc), Ok(money) => {
        assert_ne!(money, Money::none());
        assert_matches!(money.get(thb), Some(amount) => {
            assert_eq!(amount, 0);
        });
    });
}

#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let ref thb = THB();
    let ref bank = book.create_account("bank").unwrap();
    let ref wallet = book.create_account("wallet").unwrap();
    book.transfer(bank, wallet, thb.of_major(500)).unwrap();
    assert_matches!(book.balance(bank), Ok(money) => {
        assert_eq!(money, thb.of_major(-500));
    });
    assert_matches!(book.balance(wallet), Ok(money) => {
        assert_eq!(money, thb.of_major(500));
    });

    book.transfer(wallet, bank, thb.of_major(100)).unwrap();
    assert_matches!(book.balance(bank), Ok(money) => {
        assert_eq!(money, thb.of_major(-400));
    });
    assert_matches!(book.balance(wallet),Ok(money) => {
        assert_eq!(money, thb.of_major(400));
    });
}

#[test]
fn balance_at_dates() {
    let mut book = Book::new();
    let ref thb = THB();
    let ref wallet = book.create_account("wallet").unwrap();
    let ref bank = book.create_account("bank").unwrap();
    let first_withdraw_datetime = DateTime::parse_from_rfc3339("2020-11-10T10:10:57+07:00")
        .unwrap()
        .with_timezone(&Utc);
    book.transfer_at(first_withdraw_datetime, bank, wallet, thb.of_major(900))
        .unwrap();
    let second_withdraw_datetime = first_withdraw_datetime + Duration::seconds(5);
    book.transfer_at(second_withdraw_datetime, bank, wallet, thb.of_major(100))
        .unwrap();
    assert_matches!(
        book.balance_at(first_withdraw_datetime - Duration::seconds(1), wallet),
        Ok(money) => {
            assert_eq!(money, Money::none());
        },
    );
    assert_matches!(
        book.balance_at(first_withdraw_datetime, wallet),
        Ok(money) => {
            assert_eq!(money, thb.of_major(900));
        }
    );
    assert_matches!(
        book.balance_at(second_withdraw_datetime, wallet),
        Ok(money) => {
            assert_eq!(money, thb.of_major(1000));
        },
    );
}

#[test]
fn running_balance() {
    let mut book = Book::new();
    let ref thb = THB();
    let ref bank = book.create_account("bank").unwrap();
    let ref wallet = book.create_account("wallet").unwrap();
    let txs = vec![
        (wallet, bank, thb.of_major(10)),
        (bank, wallet, thb.of_major(4)),
        (bank, wallet, thb.of_major(3)),
        (bank, wallet, thb.of_major(2)),
        (bank, wallet, thb.of_major(1)),
    ];

    txs.into_iter()
        .for_each(|tx| book.transfer(tx.0, tx.1, tx.2).unwrap());

    let mut running_balance = book.running_balance(bank).unwrap().collect::<Vec<_>>();
    running_balance.sort_by_key(|(tx, _)| tx.datetime());
    assert_eq!(
        running_balance
            .iter()
            .map(|(_, m)| m.to_owned())
            .collect::<Vec<_>>(),
        [10, 6, 3, 1, 0]
            .iter()
            .map(|&n| thb.of_major(n))
            .collect::<Vec<_>>()
    );
    println!("running balance of {:?}", bank);
    running_balance.iter().for_each(|bal| {
        println!("{:?}", bal);
    });
}

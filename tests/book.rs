use chrono::Duration;
use envelope_system::book::*;


#[test]
fn adding_one_account() {
    let mut book = Book::new();
    book.new_account("wallet");
    let wallet = book.account_with_name("wallet").unwrap();
    assert_eq!(
        book.accounts().next().unwrap(),
        wallet,
        "they are the same account"
    );
}

#[test]
fn transfer_own_account() {
    let mut book = Book::new();
    book.new_account("acc");
    //let acc = book.account_with_name("acc").unwrap();
    let thb = THB();
    //assert_eq!(book.balance(&acc), Money::none());
    book.transfer("acc", "acc", thb.of_major(5));
    let _acc = book.account_with_name("acc"); //.unwrap();    
    //assert_eq!(book.balance(acc).get(&thb).unwrap(), 0);
}
/*
#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let bank = Account::new("bank");
    book.new_account(&bank);
    let wallet = Account::new("wallet");
    book.new_account(&wallet);
    let thb = THB();
    let withdraw_500_from_bank = Transaction::new(&bank, &wallet, thb.of_major(500));
    book.transfer(&withdraw_500_from_bank);
    let bank_balance = book.balance(&bank);
    let wallet_balance = book.balance(&wallet);
    assert_eq!(bank_balance.get(&thb).unwrap(), -50000);
    assert_eq!(wallet_balance.get(&thb).unwrap(), 50000);

    let put_100_into_bank = Transaction::new(&wallet, &bank, thb.of_major(100));
    book.transfer(&put_100_into_bank);
    assert_eq!(book.balance(&bank).get(&thb).unwrap(), -40000);
    assert_eq!(book.balance(&wallet).get(&thb).unwrap(), 40000);
}

#[test]
fn balance_at_dates() {
    let mut book = Book::new();
    let thb = THB();
    let wallet = Account::new("wallet");
    book.new_account(&wallet);
    let bank = Account::new("bank");
    book.new_account(&bank);
    let first_withdraw_datetime = DateTime::parse_from_rfc3339("2020-11-10T10:10:57+07:00")
        .unwrap()
        .with_timezone(&Utc);
    let withdraw_900 =
        Transaction::new_at(first_withdraw_datetime, &bank, &wallet, thb.of_major(900));
    book.transfer(&withdraw_900);
    let second_withdraw_datetime = first_withdraw_datetime + Duration::seconds(5);
    let withdraw_100 =
        Transaction::new_at(second_withdraw_datetime, &bank, &wallet, thb.of_major(100));
    book.transfer(&withdraw_100);
    assert_eq!(
        book.balance_at(first_withdraw_datetime - Duration::seconds(1), &wallet),
        Money::none()
    );
    assert_eq!(
        book.balance_at(first_withdraw_datetime, &wallet),
        thb.of_major(900)
    );
    assert_eq!(
        book.balance_at(second_withdraw_datetime, &wallet),
        thb.of_major(1000)
    );
}

#[test]
fn running_balance() {
    let mut book = Book::new();
    let thb = THB();
    let bank = Account::new("bank");
    book.new_account(&bank);
    let wallet = Account::new("wallet");
    book.new_account(&wallet);

    let txs = vec![
        Transaction::new(&wallet, &bank, thb.of_major(10)),
        Transaction::new(&bank, &wallet, thb.of_major(2)),
        Transaction::new(&bank, &wallet, thb.of_major(2)),
        Transaction::new(&bank, &wallet, thb.of_major(2)),
        Transaction::new(&bank, &wallet, thb.of_major(2)),
    ];

    txs.iter().for_each(|tx| book.transfer(tx));

    let mut running_balance = book.running_balance(&bank);
    running_balance.sort_by_key(|(tx, _)| tx.datetime());
    assert_eq!(
        running_balance
            .iter()
            .map(|(_, m)| m.get(&thb).unwrap())
            .collect::<Vec<_>>(),
        vec![1000, 800, 600, 400, 200]
    );
    println!("running balance of {:?}", bank);
    running_balance.iter().for_each(|bal| {
        println!("{:?}", bal);
    });
}
*/

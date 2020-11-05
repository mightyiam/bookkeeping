use envelope_system::{Account, Book};

#[test]
fn adding_one_account() {
    let mut book = Book::new();
    let wallet = book.new_account("wallet");
    assert_eq!(
        book.accounts().first().unwrap().as_ref() as *const Account as usize,
        wallet.as_ref() as *const Account as usize,
        "they are the same account"
    );
}

#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let _500_baht = currency::THB.of(50000);
    let take_500_baht_out_of_bank = book.transfer(bank, wallet, _500_baht);
    let balances = book.account_balances();
    assert_eq!(balances.get(bank).unwrap(), currency::THB.of(-50000));
    assert_eq!(balances.get(wallet).unwrap(), currency::THB.of(50000));
}

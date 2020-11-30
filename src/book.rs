use crate::balance::Balance;
use crate::records::{Account, Move, Unit};
use crate::sum::Sum;
use duplicate::duplicate;
use slotmap::{new_key_type, DenseSlotMap};
use std::cmp::Ordering;
use std::ops;
new_key_type! {
    /// A key type for referencing accounts.
    pub struct AccountKey;
    /// A key type for referencing units.
    pub struct UnitKey;
    /// A key type for referencing moves.
    pub struct MoveKey;
}
/// Represents a book.
#[derive(Default)]
pub struct Book<Bm, Am, Um, Mm> {
    meta: Bm,
    accounts: DenseSlotMap<AccountKey, Account<Am>>,
    units: DenseSlotMap<UnitKey, Unit<Um>>,
    moves: DenseSlotMap<MoveKey, Move<Mm>>,
}
impl<Bm, Am, Um, Mm> Book<Bm, Am, Um, Mm> {
    /// Creates a new book
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::{ DateTime, Utc };
    /// struct BookMetadata { id: u8 }
    /// struct AccountMetadata { name: String }
    /// struct UnitMetadata { currency_code: String }
    /// struct MoveMetadata { date: DateTime<Utc> }
    /// let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    ///     BookMetadata { id: 0 },
    /// );
    /// ```
    pub fn new(meta: Bm) -> Self {
        Self {
            meta,
            accounts: DenseSlotMap::<AccountKey, Account<Am>>::with_key(),
            units: DenseSlotMap::<UnitKey, Unit<Um>>::with_key(),
            moves: DenseSlotMap::<MoveKey, Move<Mm>>::with_key(),
        }
    }
    /// Gets the book's metadata.
    pub fn metadata(&self) -> &Bm {
        &self.meta
    }
    /// Sets the book's metadata.
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::{ DateTime, Utc };
    /// struct BookMetadata { id: u8 }
    /// # struct AccountMetadata { name: String }
    /// # struct UnitMetadata { currency_code: String }
    /// # struct MoveMetadata { date: DateTime<Utc> }
    /// let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    ///     BookMetadata { id: 0 },
    /// );
    /// book.set_metadata(BookMetadata{ id: 1 });
    /// ```
    pub fn set_metadata(&mut self, meta: Bm) {
        self.meta = meta;
    }
    /// Creates a new account.
    pub fn new_account(&mut self, meta: Am) -> AccountKey {
        self.accounts.insert(Account::new(meta))
    }
    /// Creates a new unit.
    pub fn new_unit(&mut self, meta: Um) -> UnitKey {
        self.units.insert(Unit::new(meta))
    }
    /// Creates a new move.
    ///
    /// ## Panics
    ///
    /// - Some of `debit_account` or `credit_account` are not in the book.
    /// - `debit_account` and `credit_account` are equal.
    /// - Some units that are in the sum are not in the book.
    pub fn new_move(
        &mut self,
        debit_account: AccountKey,
        credit_account: AccountKey,
        sum: Sum,
        meta: Mm,
    ) -> MoveKey {
        [debit_account, credit_account].iter().for_each(|key| {
            self.assert_has_account(*key);
        });
        sum.0.keys().for_each(|key| {
            self.assert_has_unit(*key);
        });
        let move_ = Move::new(debit_account, credit_account, sum, meta);
        self.moves.insert(move_)
    }
    /// Calculates the balance of an account at a move according to a provided order of moves.
    ///
    /// ## Panics
    ///
    /// - The account is not in the book.
    /// - The account is not debit nor credit in the move.
    pub fn account_balance_at_move<'a>(
        &'a self,
        account: AccountKey,
        move_: MoveKey,
        cmp: impl Fn(&Mm, &Mm) -> Ordering,
    ) -> Balance<'a> {
        self.assert_has_account(account);
        self.assert_has_move(move_);
        let move_ = self.moves.get(move_).unwrap();
        if ![move_.debit_account, move_.credit_account].contains(&account) {
            panic!(
                "Provided account is not debit nor credit in provided move."
            );
        }
        self.moves
            .iter()
            .filter(|(_, other_move)| {
                match cmp(&move_.meta, &other_move.meta) {
                    Ordering::Less => false,
                    _ => true,
                }
            })
            .filter_map(
                |(_, move_)| -> Option<(fn(&mut Balance<'a>, _), &Sum)> {
                    if move_.debit_account == account {
                        Some((ops::SubAssign::sub_assign, &move_.sum))
                    } else if move_.credit_account == account {
                        Some((ops::AddAssign::add_assign, &move_.sum))
                    } else {
                        None
                    }
                },
            )
            .fold(Balance::new(), |mut balance, (operation, sum)| {
                operation(&mut balance, sum);
                balance
            })
    }
}
#[duplicate(
    R             getter        set_metadata             assert_has           Key          M    plural      string    ;
    [Account<Am>] [get_account] [set_account_metadata]   [assert_has_account] [AccountKey] [Am] [accounts] ["account"];
    [Unit<Um>]    [get_unit]    [set_unit_metadata]      [assert_has_unit]    [UnitKey]    [Um] [units]    ["unit"]   ;
    [Move<Mm>]    [get_move]    [set_move_metadata]      [assert_has_move]    [MoveKey]    [Mm] [moves]    ["move"]   ;
)]
impl<Bm, Am, Um, Mm> Book<Bm, Am, Um, Mm> {
    /// Sets the metadata value.
    pub fn set_metadata(&mut self, key: Key, meta: M) {
        self.plural
            .get_mut(key)
            .expect("No value found for this key.")
            .meta = meta;
    }
    /// Gets a record using a key.
    ///
    /// ## Panics
    ///
    /// - No such record in the book.
    pub fn getter(&self, key: Key) -> &R {
        self.assert_has(key);
        self.plural.get(key).unwrap()
    }
    fn assert_has(&self, key: Key) {
        assert!(
            self.plural.contains_key(key),
            format!("No {} found for key {:?}", string, key),
        );
    }
    /// Gets an iterator of existing records in order of creation.
    pub fn plural(&self) -> impl Iterator<Item = (Key, &M)> {
        self.plural.iter().map(|(k, a)| (k, &a.meta))
    }
}
#[cfg(test)]
mod test {
    use super::Balance;
    use crate::sum::Sum;
    #[test]
    fn new() {
        let book = test_book!("");
        assert_eq!(book.meta, "");
        assert!(book.accounts.is_empty());
        assert!(book.units.is_empty());
        assert!(book.moves.is_empty());
    }
    #[test]
    fn new_account() {
        let mut book = test_book!("");
        book.new_account("");
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    fn new_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        assert_eq!(book.units.len(), 1,);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn new_move_panic_debit_account_not_found() {
        let mut book = test_book!("");
        let debit = book.new_account("");
        book.accounts.remove(debit);
        let credit = book.new_account("");
        book.new_move(debit, credit, Sum::new(), "");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn new_move_panic_credit_account_not_found() {
        let mut book = test_book!("");
        let debit = book.new_account("");
        let credit = book.new_account("");
        book.accounts.remove(credit);
        book.new_move(debit, credit, Sum::new(), "");
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn new_move_panic_unit_not_found() {
        let mut book = test_book!("");
        let debit = book.new_account("");
        let credit = book.new_account("");
        let unit = book.new_unit("");
        book.units.remove(unit);
        let sum = Sum::of(0, unit);
        book.new_move(debit, credit, sum, "");
    }
    #[test]
    fn new_move() {
        let mut book = test_book!("");
        let debit = book.new_account("");
        let credit = book.new_account("");
        let sum = Sum::new();
        book.new_move(debit, credit, sum, "");
        assert_eq!(book.moves.len(), 1);
    }
    #[test]
    fn accounts() {
        let mut book = test_book!("");
        assert!(book.accounts().next().is_none());
        let account = book.new_account("");
        let expected = vec![(account, &"")];
        let actual = book.accounts().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn units() {
        let mut book = test_book!("");
        assert!(book.units().next().is_none());
        let unit = book.new_unit("");
        let expected = vec![(unit, &"")];
        let actual = book.units().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn moves() {
        let mut book = test_book!("");
        assert!(book.moves().next().is_none());
        let credit_account = book.new_account("");
        let debit_account = book.new_account("");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "");
        let expected = vec![(move_, &"")];
        let actual = book.moves().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn get_account() {
        let mut book = test_book!("");
        book.new_account("");
        let account = book.new_account("!");
        book.new_account("");
        let account = book.get_account(account);
        assert_eq!(*account.metadata(), "!");
    }
    #[test]
    fn get_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        let unit = book.new_unit("!");
        book.new_unit("");
        let unit = book.get_unit(unit);
        assert_eq!(*unit.metadata(), "!");
    }
    #[test]
    fn get_move() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        book.new_move(debit_account, credit_account, Sum::new(), "");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "!");
        book.new_move(debit_account, credit_account, Sum::new(), "");
        let move_ = book.get_move(move_);
        assert_eq!(*move_.metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn assert_has_account() {
        let mut book = test_book!("");
        let account = book.new_account("");
        book.accounts.remove(account);
        book.assert_has_account(account);
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn assert_has_unit() {
        let mut book = test_book!("");
        let unit = book.new_unit("");
        book.units.remove(unit);
        book.assert_has_unit(unit);
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn assert_has_move() {
        let mut book = test_book!("");
        let credit_account = book.new_account("");
        let debit_account = book.new_account("");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "");
        book.moves.remove(move_);
        book.assert_has_move(move_);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_move_account_not_found() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "");
        book.accounts.remove(debit_account);
        book.account_balance_at_move(debit_account, move_, |_, _| panic!());
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn account_balance_at_move_move_not_found() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "");
        book.moves.remove(move_);
        book.account_balance_at_move(debit_account, move_, |_, _| panic!());
    }
    #[test]
    #[should_panic(
        expected = "Provided account is not debit nor credit in provided move."
    )]
    fn account_balance_at_move_account_not_related_to_move() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        let move_ =
            book.new_move(debit_account, credit_account, Sum::new(), "");
        let other_account = book.new_account("");
        book.account_balance_at_move(other_account, move_, |_, _| {
            panic!();
        });
    }
    #[test]
    fn account_balance_at_move() {
        let cmp = |a: &u8, b: &u8| a.cmp(&b);
        let mut book = super::Book::<&str, &str, &str, u8>::new("");
        let account_a = book.new_account("");
        let account_b = book.new_account("");
        let unit = book.new_unit("");
        let move_1 = book.new_move(account_a, account_b, Sum::of(3, unit), 1);
        assert_eq!(
            book.account_balance_at_move(account_a, move_1, cmp),
            Balance::new() - &Sum::of(3, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_1, cmp),
            Balance::new() + &Sum::of(3, unit),
        );

        let move_2 = book.new_move(account_a, account_b, Sum::of(4, unit), 2);
        assert_eq!(
            book.account_balance_at_move(account_a, move_1, cmp),
            Balance::new() - &Sum::of(3, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_1, cmp),
            Balance::new() + &Sum::of(3, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_a, move_2, cmp),
            Balance::new() - &Sum::of(7, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_2, cmp),
            Balance::new() + &Sum::of(7, unit),
        );

        let move_0 = book.new_move(account_a, account_b, Sum::of(1, unit), 0);
        assert_eq!(
            book.account_balance_at_move(account_a, move_0, cmp),
            Balance::new() - &Sum::of(1, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_0, cmp),
            Balance::new() + &Sum::of(1, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_a, move_1, cmp),
            Balance::new() - &Sum::of(4, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_1, cmp),
            Balance::new() + &Sum::of(4, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_a, move_2, cmp),
            Balance::new() - &Sum::of(8, unit),
        );
        assert_eq!(
            book.account_balance_at_move(account_b, move_2, cmp),
            Balance::new() + &Sum::of(8, unit),
        );
    }
    #[test]
    fn set_metadata() {
        let mut book = test_book!("");
        book.set_metadata("!");
        assert_eq!(book.meta, "!");
    }
    #[test]
    fn metadata() {
        let book = test_book!("!");
        assert_eq!(*book.metadata(), "!");
    }
    #[test]
    fn set_account_metadata() {
        let mut book = test_book!("");
        let account = book.new_account("");
        book.set_account_metadata(account, "!");
        assert_eq!(*book.accounts.get(account).unwrap().metadata(), "!");
    }
    #[test]
    fn set_unit_metadata() {
        let mut book = test_book!("");
        let unit = book.new_unit("");
        assert_eq!(*book.units.get(unit).unwrap().metadata(), "");
        book.set_unit_metadata(unit, "!");
        assert_eq!(*book.units.get(unit).unwrap().metadata(), "!");
    }
    #[test]
    fn set_move_metadata() {
        let mut book = test_book!("");
        let debit = book.new_account("");
        let credit = book.new_account("");
        let move_ = book.new_move(debit, credit, Sum::new(), "");
        assert_eq!(*book.moves.get(move_).unwrap().metadata(), "");
        book.set_move_metadata(move_, "!");
        assert_eq!(*book.moves.get(move_).unwrap().metadata(), "!");
    }
}

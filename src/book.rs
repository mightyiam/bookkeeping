use crate::account::Account;
use crate::balance::Balance;
use crate::move_::Move;
use crate::sum::Sum;
use crate::unit::Unit;
use duplicate::duplicate;
use slotmap::{new_key_type, DenseSlotMap};
use std::cmp::Ordering;
use std::ops;
new_key_type! {
    pub struct AccountKey;
    pub struct UnitKey;
    pub struct MoveKey;
}
enum RecordKey {
    Account(AccountKey),
    Unit(UnitKey),
    Move(MoveKey),
}
/// Represents a book.
#[derive(Default)]
pub struct Book<B, A, U, M> {
    meta: B,
    accounts: DenseSlotMap<AccountKey, Account<A>>,
    units: DenseSlotMap<UnitKey, Unit<U>>,
    moves: DenseSlotMap<MoveKey, Move<M>>,
}
impl<B, A, U, M> Book<B, A, U, M> {
    /// Creates a new book
    pub fn new(meta: B) -> Self {
        Self {
            meta,
            accounts: DenseSlotMap::<AccountKey, Account<A>>::with_key(),
            units: DenseSlotMap::<UnitKey, Unit<U>>::with_key(),
            moves: DenseSlotMap::<MoveKey, Move<M>>::with_key(),
        }
    }
    /// Gets the book's metadata.
    pub fn get_book_metadata(&self) -> &B {
        &self.meta
    }
    /// Gets the book's metadata.
    pub fn set_book_metadata(&mut self, meta: B) {
        self.meta = meta;
    }
    /// Creates a new account.
    pub fn new_account(&mut self, meta: A) -> AccountKey {
        self.accounts.insert(Account::new(meta))
    }
    /// Creates a new unit.
    pub fn new_unit(&mut self, meta: U) -> UnitKey {
        self.units.insert(Unit::new(meta))
    }
    fn assert_exists(&self, key: RecordKey) {
        let (contains, entity, key) = match key {
            RecordKey::Account(key) => (
                self.accounts.contains_key(key),
                "account",
                format!("{:?}", key),
            ),
            RecordKey::Unit(key) => (self.units.contains_key(key), "unit", format!("{:?}", key)),
            RecordKey::Move(key) => (self.moves.contains_key(key), "move", format!("{:?}", key)),
        };
        assert!(contains, format!("No {} found for key {:?}", entity, key));
    }
    /// Creates a new move.
    ///
    /// ## Panics
    ///
    /// - `debit_account` or `credit_account` are in not in the book.
    /// - `debit_account` and `credit_account` are the same.
    /// - Some [Unit][crate::Unit] in the [Sum] is not in the book.
    pub fn new_move(
        &mut self,
        debit_account: AccountKey,
        credit_account: AccountKey,
        sum: Sum,
        meta: M,
    ) -> MoveKey {
        [debit_account, credit_account].iter().for_each(|key| {
            self.assert_exists(RecordKey::Account(*key));
        });
        sum.0.keys().for_each(|key| {
            self.assert_exists(RecordKey::Unit(*key));
        });
        let move_ = Move::new(debit_account, credit_account, sum, meta);
        self.moves.insert(move_)
    }
    /// Calculates the balance of an account at a move according to a provided order of moves.
    ///
    /// ## Panics
    ///
    /// - The account is not debit nor credit in the move.
    pub fn account_balance_with_move(
        &self,
        account: AccountKey,
        move_: MoveKey,
        cmp: impl Fn(&M, &M) -> Ordering,
    ) -> Balance {
        self.assert_exists(RecordKey::Account(account));
        self.assert_exists(RecordKey::Move(move_));
        let move_ = self.moves.get(move_).unwrap();
        if ![move_.debit_account, move_.credit_account].contains(&account) {
            panic!("Provided account is not debit nor credit in provided move.");
        }
        self.moves
            .iter()
            .filter(|(_, other_move)| match cmp(&move_.meta, &other_move.meta) {
                Ordering::Less => false,
                _ => true,
            })
            .filter_map(|(_, move_)| -> Option<(fn(&mut Balance, _), &Sum)> {
                if move_.debit_account == account {
                    Some((ops::SubAssign::sub_assign, &move_.sum))
                } else if move_.credit_account == account {
                    Some((ops::AddAssign::add_assign, &move_.sum))
                } else {
                    None
                }
            })
            .fold(Balance::new(), |mut balance, (operation, sum)| {
                operation(&mut balance, sum);
                balance
            })
    }
}
#[duplicate(
    setter                 getter                 Key          T   field;
    [set_account_metadata] [get_account_metadata] [AccountKey] [A] [accounts];
    [set_unit_metadata]    [get_unit_metadata]    [UnitKey]    [U] [units];
    [set_move_metadata]    [get_move_metadata]    [MoveKey]    [M] [moves];
)]
impl<B, A, U, M> Book<B, A, U, M> {
    /// Sets the metadata value.
    pub fn setter(&mut self, key: Key, meta: T) {
        self.field
            .get_mut(key)
            .expect("No value found for this key.")
            .meta = meta;
    }
    /// Gets the metadata value on this entity.
    pub fn getter(&self, key: Key) -> &T {
        &self
            .field
            .get(key)
            .expect("No value found for this key.")
            .meta
    }
}
#[cfg(test)]
mod test {
    use super::Balance;
    use super::Book;
    use crate::sum::Sum;
    #[test]
    fn new() {
        let book = Book::<(), (), (), ()>::new(());
        assert_eq!(book.meta, ());
        assert!(book.accounts.is_empty());
        assert!(book.units.is_empty());
        assert!(book.moves.is_empty());
    }
    #[test]
    fn new_account() {
        let mut book = Book::<(), (), (), ()>::new(());
        book.new_account(());
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    fn new_unit() {
        let mut book = Book::<(), (), (), ()>::new(());
        book.new_unit(());
        assert_eq!(book.units.len(), 1,);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn new_move_panic_debit_account_not_found() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit = book.new_account(());
        book.accounts.remove(debit);
        let credit = book.new_account(());
        book.new_move(debit, credit, Sum::new(), ());
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn new_move_panic_credit_account_not_found() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        book.accounts.remove(credit);
        book.new_move(debit, credit, Sum::new(), ());
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn new_move_panic_unit_not_found() {
        let mut book = Book::<_, _, _, _>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let unit = book.new_unit(());
        book.units.remove(unit);
        let sum = Sum::of(unit, 0);
        book.new_move(debit, credit, sum, ());
    }
    #[test]
    fn new_move() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let sum = Sum::new();
        book.new_move(debit, credit, sum, ());
        assert_eq!(book.moves.len(), 1);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_move_account_not_found() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit_account = book.new_account(());
        let credit_account = book.new_account(());
        let move_ = book.new_move(debit_account, credit_account, Sum::new(), ());
        book.accounts.remove(debit_account);
        book.account_balance_with_move(debit_account, move_, |&(), &()| panic!());
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn account_balance_at_move_move_not_found() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit_account = book.new_account(());
        let credit_account = book.new_account(());
        let move_ = book.new_move(debit_account, credit_account, Sum::new(), ());
        book.moves.remove(move_);
        book.account_balance_with_move(debit_account, move_, |&(), &()| panic!());
    }
    #[test]
    #[should_panic(expected = "Provided account is not debit nor credit in provided move.")]
    fn account_balance_at_move_account_not_related_to_move() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit_account = book.new_account(());
        let credit_account = book.new_account(());
        let move_ = book.new_move(debit_account, credit_account, Sum::new(), ());
        let other_account = book.new_account(());
        book.account_balance_with_move(other_account, move_, |&(), &()| {
            panic!();
        });
    }
    #[test]
    fn account_balance_at_move() {
        use maplit::btreemap;
        let cmp = |a: &u8, b: &u8| a.cmp(&b);
        let mut book = Book::new(());
        let account_a = book.new_account(());
        let account_b = book.new_account(());
        let unit = book.new_unit(());
        let move_1 = book.new_move(account_a, account_b, Sum::of(unit, 3), 1);
        assert_eq!(
            book.account_balance_with_move(account_a, move_1, cmp),
            Balance(btreemap! { unit.clone() => -3 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_1, cmp),
            Balance(btreemap! { unit.clone() => 3 }),
        );

        let move_2 = book.new_move(account_a, account_b, Sum::of(unit, 4), 2);
        assert_eq!(
            book.account_balance_with_move(account_a, move_1, cmp),
            Balance(btreemap! { unit.clone() => -3 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_1, cmp),
            Balance(btreemap! { unit.clone() => 3 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_a, move_2, cmp),
            Balance(btreemap! { unit.clone() => -7 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_2, cmp),
            Balance(btreemap! { unit.clone() => 7 }),
        );

        let move_0 = book.new_move(account_a, account_b, Sum::of(unit, 1), 0);
        assert_eq!(
            book.account_balance_with_move(account_a, move_0, cmp),
            Balance(btreemap! { unit.clone() => -1 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_0, cmp),
            Balance(btreemap! { unit.clone() => 1 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_a, move_1, cmp),
            Balance(btreemap! { unit.clone() => -4 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_1, cmp),
            Balance(btreemap! { unit.clone() => 4 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_a, move_2, cmp),
            Balance(btreemap! { unit.clone() => -8 }),
        );
        assert_eq!(
            book.account_balance_with_move(account_b, move_2, cmp),
            Balance(btreemap! { unit.clone() => 8 }),
        );
    }
    #[test]
    fn metadata() {
        let mut book = Book::<_, (), (), ()>::new(3);
        assert_eq!(*book.get_book_metadata(), 3);
        book.set_book_metadata(20);
        assert_eq!(*book.get_book_metadata(), 20);
        book.set_book_metadata(9);
        assert_eq!(*book.get_book_metadata(), 9);
    }
    #[test]
    fn set_account_metadata() {
        let mut book = Book::<_, _, (), ()>::new(());
        let account = book.new_account(3);
        assert_eq!(*book.get_account_metadata(account), 3);
        book.set_account_metadata(account, 5);
        assert_eq!(*book.get_account_metadata(account), 5);
    }
    #[test]
    fn set_unit_metadata() {
        let mut book = Book::<_, (), _, ()>::new(());
        let unit = book.new_unit(3);
        assert_eq!(*book.get_unit_metadata(unit), 3);
        book.set_unit_metadata(unit, 5);
        assert_eq!(*book.get_unit_metadata(unit), 5);
    }
    #[test]
    fn set_move_metadata() {
        let mut book = Book::<_, _, (), _>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let move_ = book.new_move(debit, credit, Sum::new(), 7);
        assert_eq!(*book.get_move_metadata(move_), 7);
        book.set_move_metadata(move_, 5);
        assert_eq!(*book.get_move_metadata(move_), 5);
    }
}

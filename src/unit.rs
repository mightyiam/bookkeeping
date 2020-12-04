/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<U> {
    pub(crate) meta: U,
}
impl<U> Unit<U> {
    /// Creates a new unit.
    pub(crate) fn new(meta: U) -> Self {
        Self { meta }
    }
    /// Gets the metadata of the unit.
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::naive::NaiveDate;
    /// # struct BookMetadata { id: u8 }
    /// # struct AccountMetadata { name: String }
    /// # #[derive(Debug, PartialEq)]
    /// # struct UnitMetadata { currency_code: String }
    /// # struct MoveMetadata { date: NaiveDate }
    /// # let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    /// #     BookMetadata { id: 0 },
    /// # );
    /// # let unit_key = book.new_unit(UnitMetadata { currency_code: String::from("USD") });
    /// # let unit = book.get_unit(unit_key);
    /// assert_eq!(
    ///     unit.metadata(),
    ///     &UnitMetadata { currency_code: String::from("USD") },
    /// );
    /// ```
    pub fn metadata(&self) -> &U {
        &self.meta
    }
}
#[cfg(test)]
mod test {
    use super::Unit;
    #[test]
    fn metadata() {
        let unit = Unit::new(5);
        assert_eq!(*unit.metadata(), 5);
    }
}

/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<U> {
    pub(crate) metadata: U,
}
impl<U> Unit<U> {
    /// Gets the metadata of the unit.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let unit_key = book.new_unit("USD");
    /// # let unit = book.get_unit(unit_key);
    /// assert_eq!(*unit.metadata(), "USD");
    /// ```
    pub fn metadata(&self) -> &U {
        &self.metadata
    }
}
#[cfg(test)]
mod test {
    use super::Unit;
    #[test]
    fn metadata() {
        let unit = Unit { metadata: 5 };
        assert_eq!(*unit.metadata(), 5);
    }
}

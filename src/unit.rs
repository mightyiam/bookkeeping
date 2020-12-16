/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
pub struct Unit<UnitMetadata> {
    pub(crate) metadata: UnitMetadata,
}
impl<UnitMetadata> Unit<UnitMetadata> {
    /// Gets the metadata of the unit.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::*;
    /// # let mut book = Book::<(), (), &str, (), ()>::new(());
    /// # let unit_key = book.new_unit("USD");
    /// # let unit = book.get_unit(unit_key);
    /// assert_eq!(*unit.metadata(), "USD");
    /// ```
    pub fn metadata(&self) -> &UnitMetadata {
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

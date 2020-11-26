/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($Original:ident, $Concrete:ident) => {
        type $Concrete = $Original<u8, u8, u8, u8>;
    };
}

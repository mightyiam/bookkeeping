/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($meta:expr) => {
        crate::book::Book::<u8, u8, u8, u8>::new($meta)
    };
}

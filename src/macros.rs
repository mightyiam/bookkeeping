/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($meta:expr) => {
        crate::book::Book::<&str, &str, &str, &str>::new($meta)
    };
}

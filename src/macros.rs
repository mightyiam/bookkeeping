/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($metadata:expr) => {
        crate::book::Book::<&str, &str, &str, &str, &str>::new($metadata)
    };
}
#[cfg(test)]
macro_rules! sum {
    ($($amount:expr, $unit:ident);*) => {{
        let mut sum = crate::sum::Sum::new();
        $(sum.set_amount_for_unit($amount, $unit);)*
        sum
    }}
}

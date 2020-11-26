/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($Original:ident, $Concrete:ident) => {
        slotmap::new_key_type! { struct KA; struct KU; struct KM; }
        type $Concrete = $Original<KA, KU, KM, u8, u8, u8, u8>;
    };
}

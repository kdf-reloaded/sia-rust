mod tx_builder;
pub use tx_builder::*;

use serde::{Deserialize, Deserializer};

/// Run a block of unit tests on both wasm32 and non-wasm32 targets
/// Can only be used once per scope due to wasm_bindgen initialization
#[cfg(test)]
macro_rules! cross_target_tests {
    ($($test_fn:item)*) => {
        #[cfg(all(test, target_arch = "wasm32"))]
        use wasm_bindgen_test::*;

        #[cfg(all(test, target_arch = "wasm32"))]
        wasm_bindgen_test_configure!(run_in_browser);

        $(
            #[cfg(all(test, target_arch = "wasm32"))]
            #[wasm_bindgen_test]
            $test_fn

            #[cfg(not(target_arch = "wasm32"))]
            #[test]
            $test_fn
        )*
    };
}

/// Deserialize a null value as an empty vector.
/// Allows using Vec<> instead of Option<Vec<>> for convenience.
pub(crate) fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::deserialize(deserializer).map(|opt| opt.unwrap_or_default())
}

/// Deserialize a list one element at a time and leave out an element that does not parse. A null list is an empty
/// list, as for [`deserialize_null_as_empty_vec`].
///
/// A reader of a list that other nodes' transactions end up in (the transaction pool) must not lose the whole list
/// to one element it cannot read, for example a transaction kind or field this library does not know yet. Each left
/// out element is reported on the debug log with its position in the list and the kind of its error, never its
/// content.
pub(crate) fn deserialize_tolerant_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let elements: Option<Vec<serde_json::Value>> = Option::deserialize(deserializer)?;
    let mut parsed = Vec::new();
    for (index, element) in elements.unwrap_or_default().into_iter().enumerate() {
        match serde_json::from_value::<T>(element) {
            Ok(value) => parsed.push(value),
            Err(e) => log::debug!(
                "deserialize_tolerant_vec: left out element {} of a list of {}: {:?} error",
                index,
                std::any::type_name::<T>(),
                e.classify()
            ),
        }
    }
    Ok(parsed)
}

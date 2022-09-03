use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustadd2;
    fn add;
}

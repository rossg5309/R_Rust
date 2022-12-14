use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn rust_sort(v: &[i32]) -> Vec<i32> {
    let mut sorted_v = v.to_vec();
    sorted_v.sort_unstable();
    sorted_v
}

/* fn rust_sort(x: i32, y: i32, z: i32) -> 32{
    v.sort_unstable();
} */

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustsort;
    fn rust_sort;
}

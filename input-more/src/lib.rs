#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    pointer_structural_match,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

/*
    TODO:
        Deal with pointer and non-pointer events in mapping
        Ctrl+LMB (+coords)
        Ctrl+Shift+LMB (-coords)
*/

mod event;
mod mapping;
mod state;

pub use event::*;
pub use mapping::*;
pub use state::*;

pub use input_core;

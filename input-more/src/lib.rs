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

mod binding;
mod event;
mod global_state;
mod mapping;
mod mapping_cache;
mod marker;
mod modifiers_cache;
mod state;
mod struct_take_and_with_field;
mod struct_take_field;
mod struct_with_field;

pub use binding::*;
pub use event::*;
pub use global_state::*;
pub use mapping::*;
pub use mapping_cache::*;
pub use marker::*;
pub use modifiers_cache::*;
pub use state::*;
pub use struct_take_and_with_field::*;
pub use struct_take_field::*;
pub use struct_with_field::*;

pub use input_core;

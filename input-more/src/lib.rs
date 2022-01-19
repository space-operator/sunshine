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
mod device_state;
mod event;
mod global_mapping;
mod global_mapping_cache;
mod global_state;
mod mapping;
mod mapping_cache;
mod mapping_modifiers_cache;
mod switch_mapping_cache;
mod unwrap_or;

pub use binding::*;
pub use device_state::*;
pub use event::*;
pub use global_mapping::*;
pub use global_mapping_cache::*;
pub use global_state::*;
pub use mapping::*;
pub use mapping_cache::*;
pub use mapping_modifiers_cache::*;
pub use switch_mapping_cache::*;
pub use unwrap_or::*;

pub use input_core;

// Issues:
//   coords => context
//   how to create binding for any relase event but not only for click | dblclick | longpress
//   tripleclick => emit dblclick and singleclick is we do not handle triple click
//   keyboard shortcut "create node at mouse position", how do we obtain coords
//   corrent move move thresholds

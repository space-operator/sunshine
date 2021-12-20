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

pub mod cons_ext;
//mod event;
mod modifiers;
mod processor;
mod scheduler;
//mod state;
mod timed_event;

//pub use event::*;
pub use modifiers::*;
pub use processor::*;
pub use scheduler::*;
//pub use state::*;
pub use cons_ext::*;
pub use timed_event::*;

pub use input_core;

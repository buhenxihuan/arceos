//! Data structures and operations for managing memory mappings.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod area;
mod set;

#[cfg(test)]
mod tests;

pub use self::area::{MappingBackend, MemoryArea};
pub use self::set::MemorySet;

/// Error type for memory mapping operations.
#[derive(Debug, Eq, PartialEq)]
pub enum MappingError {
    /// Invalid parameter (e.g., `addr`, `size`, `flags`, etc.)
    InvalidParam,
    /// The given range clashes with an existing.
    AlreadyExists,
    /// The backend page table is in a bad state.
    BadState,
}

/// A [`Result`] type with [`MappingError`] as the error type.
pub type MappingResult<T = ()> = Result<T, MappingError>;

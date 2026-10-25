//! Synthetic Brain Architecture portable software layer.

pub mod hal;
pub mod runtime;
pub mod cognitive;

pub use cognitive::*;
pub use hal::*;
pub use runtime::*;

//! Portable hardware abstraction layer for the synthetic-brain runtime.
//!
//! The same brain software can execute against physical hardware, a Raspberry
//! Pi 3 development target, or the virtual hardware simulator.

pub mod traits;
pub mod virtual_hardware;

#[cfg(feature = "raspberry-pi3")]
pub mod raspberry_pi3;

pub use traits::*;
pub use virtual_hardware::*;

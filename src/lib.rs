#![cfg_attr(docsrs, feature(doc_cfg))]

/// Structurally-pinned wrappers for `std::sync`'s Mutex types.
pub mod std;

/// Structurally-pinned wrappers for `parking_lot`'s Mutex types.
#[cfg_attr(docsrs, doc(cfg(feature = "parking_lot")))]
#[cfg(feature = "parking_lot")]
pub mod parking_lot;

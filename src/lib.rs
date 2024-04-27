#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod std;

#[cfg_attr(docsrs, doc(cfg(feature = "parking_lot")))]
#[cfg(feature = "parking_lot")]
pub mod parking_lot;

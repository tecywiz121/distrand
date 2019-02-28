//! # Distrand
//!
//! Distrand is a library for ***dist***tributed ***rand***om value generation.
//!
//! It uses a simple commit-reveal algorithm that is suitable for small numbers
//! of participants. Each participant must communicate with every other
//! participant, so the number of messages increases dramatically as the number
//! of participants increases.
//!
//! See the `examples` directory for a tutorial.

#![deny(missing_docs)]

extern crate bincode;
extern crate crypto_mac;
#[macro_use]
extern crate error_chain;
pub extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate typenum;

mod commit;
pub mod errors;
mod exchange;
mod reveal;
mod secret;

pub use commit::Commit;
pub use exchange::Exchange;
pub use reveal::Reveal;
pub use secret::Secret;

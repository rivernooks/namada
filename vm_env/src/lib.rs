//! This crate contains library code for wasm. Some of the code is re-exported
//! from the `shared` crate.

#![doc(html_favicon_url = "https://docs.anoma.network/favicon.png")]
#![doc(html_logo_url = "https://docs.anoma.network/rustdoc-logo.png")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

#[cfg(feature = "ibc")]
pub mod ibc;
pub mod imports;
pub mod intent;
pub mod key;
pub mod proof_of_stake;
pub mod token;

pub mod tx_prelude {
    pub use anoma::proto::{Signed, SignedTxData};
    pub use anoma::types::address::Address;
    pub use anoma::types::*;
    pub use anoma_macros::transaction;

    #[cfg(feature = "ibc")]
    pub use crate::ibc::{Ibc, IbcActions};
    pub use crate::imports::tx::*;
    pub use crate::intent::tx as intent;
    pub use crate::proof_of_stake::{self, PoS, PosRead, PosWrite};
    pub use crate::token::tx as token;
}

pub mod vp_prelude {
    // used in the VP input
    pub use std::collections::HashSet;

    pub use anoma::ledger::pos as proof_of_stake;
    pub use anoma::proto::{Signed, SignedTxData};
    pub use anoma::types::address::Address;
    pub use anoma::types::*;
    pub use anoma_macros::validity_predicate;

    pub use crate::imports::vp::*;
    pub use crate::intent::vp as intent;
    pub use crate::key::vp as key;
    pub use crate::token::vp as token;
}

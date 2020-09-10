mod account;
mod error;
mod number;
pub use account::Account;
pub use error::*;
pub use number::*;
pub mod transaction;
use crate::core::bitcoin::{bitcoin, Script};
use orga::encoding::{self as ed, Decode, Encode};

pub type Address = [u8; 33];
pub type Signature = [u8; 64];

#[derive(Clone, Encode, Decode)]
pub struct Withdrawal {
    pub value: u64,
    pub script: Script,
}

impl Into<bitcoin::TxOut> for Withdrawal {
    fn into(self) -> bitcoin::TxOut {
        bitcoin::TxOut {
            value: self.value,
            script_pubkey: self.script.into(),
        }
    }
}
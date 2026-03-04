use crate::treepp::*;
use bitcoin::TapSighashType;

/// Gadget for the hash type.
pub struct HashTypeGadget;

impl HashTypeGadget {
    /// Construct the hash type from constant data.
    pub fn from_constant(hash_type: &TapSighashType) -> Script {
        match hash_type {
            TapSighashType::Default => {
                script! {
                    OP_PUSHBYTES_1 OP_PUSHBYTES_0
                }
            }
            TapSighashType::All => {
                script! {
                    OP_PUSHNUM_1
                }
            }
            TapSighashType::None => {
                script! {
                    OP_PUSHNUM_2
                }
            }
            TapSighashType::Single => {
                script! {
                    OP_PUSHNUM_3
                }
            }
            TapSighashType::AllPlusAnyoneCanPay => {
                script! {
                    // Can't push [0x81] directly: CheckMinimalPush requires OP_1NEGATE
                    // for [0x81], but OP_1NEGATE is a NOP in Tapscript V2.
                    // Compute 128 + 1 = 129 = Val64 [0x81] instead.
                    { 128 } OP_1 OP_ADD
                }
            }
            TapSighashType::NonePlusAnyoneCanPay => {
                script! {
                    OP_PUSHBYTES_1 OP_SIZE
                }
            }
            TapSighashType::SinglePlusAnyoneCanPay => {
                script! {
                    OP_PUSHBYTES_1 OP_INVERT
                }
            }
        }
    }

    /// Construct the hash type from the provided hash type on the stack.
    ///
    /// It checks if the hash type is one of the valid ones.
    pub fn from_provided() -> Script {
        script! {
            OP_DUP OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_EQUAL
            OP_OVER OP_PUSHNUM_1 OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHNUM_2 OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHNUM_3 OP_EQUAL OP_BOOLOR
            OP_OVER { 128 } OP_1 OP_ADD OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_SIZE OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_INVERT OP_EQUAL OP_BOOLOR
            OP_VERIFY
        }
    }
}

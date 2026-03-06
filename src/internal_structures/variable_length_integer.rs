use crate::treepp::*;
use bitcoin::consensus::Encodable;
use bitcoin::opcodes::all::{
    OP_PUSHBYTES_1, OP_PUSHBYTES_3, OP_PUSHBYTES_5, OP_PUSHBYTES_9, OP_PUSHNUM_1,
};

/// Gadget for variable length integer used in Bitcoin consensus encoding.
pub struct VariableLengthIntegerGadget;

impl VariableLengthIntegerGadget {
    /// Construct the variable length integer from a Bitcoin integer on the stack
    /// that is smaller than 128.
    pub fn from_small_bitcoin_number() -> Script {
        script! {
            // making sure the number is smaller than 128
            OP_DUP
            { 128 } OP_LESSTHAN OP_VERIFY
        }
    }

    /// Construct the variable length integer from constant data.
    pub fn from_constant(v: usize) -> Script {
        let vi = bitcoin::VarInt::from(v as u64);

        let mut bytes = vec![];
        vi.consensus_encode(&mut bytes).unwrap();

        if v > 0 && v <= 16 {
            Script::from_bytes(vec![OP_PUSHNUM_1.to_u8() + (v as u8 - 1)])
        } else if vi.size() == 1 && bytes[0] == 0x81 {
            // Can't push [0x81] directly: CheckMinimalPush requires OP_1NEGATE
            // for [0x81], but OP_1NEGATE is a NOP in Tapscript V2.
            // Compute 128 + 1 = 129 = Val64 [0x81] instead.
            Script::from_bytes(vec![
                OP_PUSHBYTES_1.to_u8(),
                0x80,                 // push 128
                OP_PUSHNUM_1.to_u8(), // push 1
                0x93,                 // OP_ADD: 128 + 1 = 129
            ])
        } else if vi.size() == 1 {
            Script::from_bytes(vec![OP_PUSHBYTES_1.to_u8(), bytes[0]])
        } else if vi.size() == 3 {
            Script::from_bytes(vec![OP_PUSHBYTES_3.to_u8(), bytes[0], bytes[1], bytes[2]])
        } else if vi.size() == 5 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_5.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
            ])
        } else if vi.size() == 9 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_9.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
                bytes[5],
                bytes[6],
                bytes[7],
                bytes[8],
            ])
        } else {
            unreachable!()
        }
    }
}

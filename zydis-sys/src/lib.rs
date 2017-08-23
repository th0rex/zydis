#![feature(try_from)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryFrom;

// Some #defined symbols we have to implement manually, because bindgen has no way
// of knowing the type of the defined constant.

pub const ZYDIS_FMTFLAG_UPPERCASE: ZydisFormatterFlags = 0x1;
pub const ZYDIS_FMTFLAG_FORCE_SEGMENTS: ZydisFormatterFlags = 0x2;
pub const ZYDIS_FMT_FLAG_FORCE_OPERANDSIZE: ZydisFormatterFlags = 0x4;

macro_rules! check {
    ($expression:expr, $ok:expr) => {
        match $expression as _ {
            ZYDIS_STATUS_SUCCESS => Ok($ok),
            e => Err(e),
        }
    };
}

impl TryFrom<ZydisDecodedInstruction> for ZydisEncoderRequest {
    type Error = ZydisStatusCode;

    fn try_from(value: ZydisDecodedInstruction) -> Result<Self, ZydisStatusCode> {
        unsafe {
            let mut ret = std::mem::uninitialized();
            check!(
                ZydisEncoderDecodedInstructionToRequest(&value, &mut ret),
                ret
            )
        }
    }
}

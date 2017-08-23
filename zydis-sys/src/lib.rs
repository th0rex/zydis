#![feature(try_from)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryFrom;

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
            check!(ZydisEncoderDecodedInstructionToRequest(&value, &mut ret), ret)
        }
    }
}

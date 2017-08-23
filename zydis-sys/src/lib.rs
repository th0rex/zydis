#![feature(try_from)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryFrom;
use std::ffi::CStr;
use std::ptr::null;

// Some #defined symbols we have to implement manually, because bindgen has no way
// of knowing the type of the defined constant.

pub const ZYDIS_FMTFLAG_UPPERCASE: ZydisFormatterFlags = 0x1;
pub const ZYDIS_FMTFLAG_FORCE_SEGMENTS: ZydisFormatterFlags = 0x2;
pub const ZYDIS_FMT_FLAG_FORCE_OPERANDSIZE: ZydisFormatterFlags = 0x4;

#[macro_export]
macro_rules! check {
    ($expression:expr, $ok:expr) => {
        match $expression as _ {
            ZYDIS_STATUS_SUCCESS => Ok($ok),
            e => Err(e),
        }
    };
    (@option $expression:expr, $ok:expr) => {
        match $expression as _ {
            ZYDIS_STATUS_SUCCESS => Ok(Some($ok)),
            ZYDIS_STATUS_NO_MORE_DATA => Ok(None),
            e => Err(e),
        }
    };
    (@string $expression:expr) => {
        match $expression {
            x if x == null() => None,
            x => Some(CStr::from_ptr(x).to_str().unwrap())
        }
    }
}

impl ZydisDecodedInstruction {
    pub fn calc_absolute_target_address(&self, operand: &ZydisDecodedOperand) -> Result<u64, ZydisStatusCode> {
        unsafe {
            let mut address = 0u64;
            check!(ZydisUtilsCalcAbsoluteTargetAddress(self, operand, &mut address), address)
        }
    }
}

/// Extensions for `ZydisMnemonic`
pub trait ZydisMnemonicMethods {
    fn get_string(self) -> Option<&'static str>;
}

impl ZydisMnemonicMethods for ZydisMnemonic {
    fn get_string(self) -> Option<&'static str> {
        unsafe { check!(@string ZydisMnemonicGetString(self)) }
    }
}

/// Extensions for `ZydisRegister`
pub trait ZydisRegisterMethods {
    fn get_id(self) -> Option<i16>;

    fn get_class(self) -> ZydisRegisterClass;

    fn get_string(self) -> Option<&'static str>;

    fn get_width(self) -> ZydisRegisterWidth;

    fn get_width64(self) -> ZydisRegisterWidth;
}

impl ZydisRegisterMethods for ZydisRegister {
    fn get_id(self) -> Option<i16> {
        unsafe {
            match ZydisRegisterGetId(self) {
                -1 => None,
                x => Some(x),
            }
        }
    }

    fn get_class(self) -> ZydisRegisterClass {
        unsafe { ZydisRegisterGetClass(self) }
    }

    fn get_string(self) -> Option<&'static str> {
        unsafe { check!(@string ZydisRegisterGetString(self)) }
    }

    fn get_width(self) -> ZydisRegisterWidth {
        unsafe { ZydisRegisterGetWidth(self) }
    }

    fn get_width64(self) -> ZydisRegisterWidth {
        unsafe { ZydisRegisterGetWidth64(self) }
    }
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

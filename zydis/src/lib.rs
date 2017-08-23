extern crate zydis_sys;

use std::mem::uninitialized;
use std::os::raw::c_void;

pub use zydis_sys::*;

pub type Result<T> = std::result::Result<T, ZydisStatusCode>;

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
    }
}

pub fn encode_instruction(buffer: &mut [u8], request: &ZydisEncoderRequest) -> Result<usize> {
    unsafe {
        let mut len = buffer.len();
        check!(ZydisEncoderEncodeInstruction(buffer.as_ptr() as _, &mut len, request), len)
    }
}

pub struct Decoder {
    decoder: ZydisDecoder,
}

impl Decoder {
    pub fn new(mode: ZydisMachineMode, width: ZydisAddressWidth) -> Result<Decoder> {
        unsafe {
            let mut decoder = uninitialized();
            check!(ZydisDecoderInit(&mut decoder, mode, width), Decoder{decoder})
        }
    }

    pub fn new_granularity(mode: ZydisMachineMode, width: ZydisAddressWidth, granularity: ZydisDecodeGranularity) -> Result<Decoder> {
        unsafe {
            let mut decoder = uninitialized();
            check!(ZydisDecoderInitEx(&mut decoder, mode, width, granularity), Decoder{decoder})
        }
    }

    pub fn decode_buffer(&self, buffer: &[u8], ip: u64) -> Result<Option<ZydisDecodedInstruction>> {
        unsafe {
            let mut instruction = uninitialized();
            check!(@option ZydisDecoderDecodeBuffer(&self.decoder, buffer.as_ptr() as _, buffer.len(), ip, &mut instruction), instruction)
        }
    }

    pub fn instruction_iterator<'a, 'b>(&'a self, buffer: &'b [u8], ip: u64) -> InstructionIterator<'a, 'b> {
        InstructionIterator{
            decoder: self,
            buffer,
            ip,
        }
    }
}

pub struct InstructionIterator<'a, 'b> {
    decoder: &'a Decoder,
    buffer: &'b [u8],
    ip: u64,
}

impl<'a, 'b> Iterator for InstructionIterator<'a, 'b> {
    type Item = ZydisDecodedInstruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.decoder.decode_buffer(self.buffer, self.ip) {
            Ok(Some(insn)) => {
                self.buffer = &self.buffer[insn.length as usize..];
                Some(insn)
            }
            _ => None,
        }
    }
}

pub struct Formatter {
    formatter: ZydisFormatter,
}

impl Formatter {
    pub fn new(style: ZydisFormatterStyle) -> Result<Formatter> {
        unsafe {
            let mut formatter = uninitialized();
            check!(ZydisFormatterInit(&mut formatter, style), Formatter{formatter})
        }
    }

    pub fn new_extended(style: ZydisFormatterStyle, flags: ZydisFormatterFlags, address_format: ZydisFormatterAddressFormat, displacement_format: ZydisFormatterDisplacementFormat, immediate_format: ZydisFormatterImmediateFormat) -> Result<Formatter> {
        unsafe {
            let mut formatter = uninitialized();
            check!(ZydisFormatterInitEx(&mut formatter, style, flags, address_format, displacement_format, immediate_format), Formatter{formatter})
        }
    }

    pub fn format_instruction(&self, mut instruction: ZydisDecodedInstruction, buffer: &mut [u8]) -> Result<()> {
        unsafe {
            check!(ZydisFormatterFormatInstruction(&self.formatter, &mut instruction, buffer.as_ptr() as _, buffer.len()), ())
        }
    }

    pub fn set_hook(&mut self, hook: ZydisFormatterHookType, callback: usize) -> Result<usize> {
        unsafe {
            let mut callback = callback as *const c_void;
            check!(ZydisFormatterSetHook(&mut self.formatter, hook, &mut callback), callback as _)
        }
    }
}

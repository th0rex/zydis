extern crate zydis;

use std::mem::transmute;
use std::os::raw::c_char;

use zydis::*;

static mut DEFAULT_FORMAT_IMM: ZydisFormatterFormatOperandFunc = None;

extern "C" fn format_operand_imm(formatter: *const ZydisFormatter, buffer: *mut *mut c_char, len: usize, instruction: *mut ZydisDecodedInstruction, operand: *mut ZydisDecodedOperand) -> ZydisStatus {
    println!("formatting operand ...");
    unsafe { DEFAULT_FORMAT_IMM.unwrap()(formatter, buffer, len, instruction, operand) }
}

fn main() {
    let data = &[
        0x68u8,
        0x34,
        0x12,
        0x00,
        0x00,
        0xb8,
        0x76,
        0x98,
        0x00,
        0x00,
        0xe8,
        0x35,
        0x33,
        0x22,
        0x11,
    ];

    let decoder = Decoder::new(ZYDIS_MACHINE_MODE_LONG_64, ZYDIS_ADDRESS_WIDTH_64).expect("Could not create decoder");
    let mut formatter = Formatter::new(ZYDIS_FORMATTER_STYLE_INTEL).expect("Could not create formatter");

    unsafe {
        DEFAULT_FORMAT_IMM = Some(transmute(formatter.set_hook(ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_IMM, format_operand_imm as _).unwrap()))
    };

    for ins in decoder.instruction_iterator(data, 0x0) {

    }
}
extern crate zydis;

use std::ffi::CStr;

use zydis::*;

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

    let decoder = Decoder::new(ZYDIS_MACHINE_MODE_LONG_64, ZYDIS_ADDRESS_WIDTH_64)
        .expect("Could not create decoder");
    let formatter =
        Formatter::new(ZYDIS_FORMATTER_STYLE_INTEL).expect("Could not create formatter");

    for ins in decoder.instruction_iterator(data, 0x0) {
        let mut buffer = [0u8; 256];
        formatter
            .format_instruction(ins, &mut buffer)
            .expect("Could not format instruction");

        let string = unsafe { CStr::from_ptr(buffer.as_ptr() as _) }
            .to_str()
            .unwrap();
        println!("instruction: {}", string);

        // Alternatively use this function, which allocates on the heap and returns an owned string.
        // The function does assume that zydis returns valid utf8 and panics otherwise.
        println!(
            "instruction: {}",
            formatter.format_instruction_str(ins, 256).unwrap()
        );
        assert_eq!(
            formatter
                .format_instruction_str(ins, 256)
                .unwrap()
                .chars()
                .find(|&c| c == '\n'),
            None
        );
    }
}

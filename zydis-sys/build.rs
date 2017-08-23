extern crate bindgen;
extern crate gcc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bindgen::Builder;
use gcc::Build;

const ZYDIS_INCLUDE_PATH: &'static str = "third_party/zydis/include";
const ZYDIS_SRC_PATH: &'static str = "third_party/zydis/src";

fn build_library() {
    Build::new()
        .include(ZYDIS_INCLUDE_PATH)
        .include(ZYDIS_SRC_PATH)
        // "ZydisExportConfig.h" is placed in the current dir.
        // It only fakes the `generate_export_header` from CMake.
        .include(env::current_dir().unwrap())
        .define("ZYDIS_ENABLE_FEATURE_EVEX", None)
        .define("ZYDIS_ENABLE_FEATURE_MVEX", None)
        .define("ZYDIS_ENABLE_FEATURE_FLAGS", None)
        .define("ZYDIS_ENABLE_FEATURE_DECODER", None)
        .define("ZYDIS_ENABLE_FEATURE_ENCODER", None)
        .files(vec![
            format!("{}/Decoder.c", ZYDIS_SRC_PATH),
            format!("{}/DecoderData.c", ZYDIS_SRC_PATH),
            format!("{}/Encoder.c", ZYDIS_SRC_PATH),
            format!("{}/EncoderData.c", ZYDIS_SRC_PATH),
            format!("{}/Formatter.c", ZYDIS_SRC_PATH),
            format!("{}/Mnemonic.c", ZYDIS_SRC_PATH),
            format!("{}/Register.c", ZYDIS_SRC_PATH),
            format!("{}/SharedData.c", ZYDIS_SRC_PATH),
            format!("{}/Utils.c", ZYDIS_SRC_PATH),
            format!("{}/Zydis.c", ZYDIS_SRC_PATH),
        ])
        .compile("libzydis.a");
}

fn build_bindings(out_path: PathBuf, wrapper: PathBuf) {
    let bindings = Builder::default()
        .unstable_rust(true)
        .header(wrapper.to_str().unwrap())
        .clang_arg(format!("-I{}", ZYDIS_INCLUDE_PATH))
        .clang_arg(format!("-I{}", ZYDIS_SRC_PATH))
        .clang_arg(format!("-I{}", env::current_dir().unwrap().to_str().unwrap()))
        .emit_builtins()
        .link("zydis")
        .constified_enum("Zydis.*")
        .whitelisted_type("Zydis.*")
        .whitelisted_function("Zydis.*")
        .layout_tests(true)
        .prepend_enum_name(false)
        .generate()
        .expect("Could not generate bindings to zydis");

    bindings.write_to_file(out_path.join("bindings.rs")).expect("Could not write bindings");
}

fn main() {
    println!("cargo:rerun-if-changed=third_party/zydis");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let wrapper = out_path.join("wrapper.h");
    {
        let mut file = File::create(&wrapper).expect("Couldn't create wrapper file in output directory");
        file.write_all(b"#include <Zydis/Zydis.h>").expect("Couldn't write wrapper code to wrapper file");
    }

    build_library();
    build_bindings(out_path, wrapper);
}
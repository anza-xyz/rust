use crate::abi::Endian;
use super::{LinkerFlavor, PanicStrategy, Target, TargetOptions, LldFlavor};
use std::{collections::BTreeMap};

pub fn target() -> Target {
    let linker_script = r"
PHDRS
{
  text PT_LOAD ;
  rodata PT_LOAD ;
  dynamic PT_DYNAMIC ;
}

SECTIONS
{
  . = SIZEOF_HEADERS;
  .text : { *(.text*) } :text
  .rodata : { *(.rodata*) } :rodata
  .data.rel.ro : { *(.data.rel.ro*) } :rodata
  .dynamic : { *(.dynamic) } :dynamic
}
";
    let mut lld_args = Vec::new();
    lld_args.push("--threads=1".to_string());
    lld_args.push("-z".to_string());
    lld_args.push("notext".to_string());
    let mut pre_link_args = BTreeMap::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), lld_args);

    Target {
        llvm_target: "bpfel".to_string(),
        pointer_width: 64,
        arch: "bpf".to_string(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".to_string(),

        options: TargetOptions {
            allow_asm: true,
            endian: Endian::Little,
            c_int_width: "64".to_string(),
            env: String::new(),
            features: "+solana".to_string(),
            vendor: "solana".to_string(),
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker_is_gnu: true,
            linker: Some("rust-lld".to_owned()),
            link_script: Some(linker_script.to_string()),
            pre_link_args,
            executables: true,
            dll_prefix: "".to_string(),
            dynamic_linking: true,
            only_cdylib: true,
            no_default_libraries: true,
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            requires_lto: false,
            singlethread: true,
            max_atomic_width: Some(64),
            eh_frame_header: false,
            main_needs_argc_argv: false,
            emit_debug_gdb_scripts: false,
            .. Default::default()
        },
    }
}

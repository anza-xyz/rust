use crate::abi::Endian;
use super::{LinkerFlavor, PanicStrategy, TargetOptions, LldFlavor};
use std::{collections::BTreeMap};

pub fn opts() -> TargetOptions {
    let linker_script = r"
PHDRS
{
  text PT_LOAD ;
  rodata PT_LOAD ;
  data PT_LOAD ;
  dynamic PT_DYNAMIC ;
}

SECTIONS
{
  . = SIZEOF_HEADERS;
  .text : { *(.text*) } :text
  .rodata : { *(.rodata*) } :rodata
  .data.rel.ro : { *(.data.rel.ro*) } :rodata
  .dynamic : { *(.dynamic) } :dynamic
  .dynsym : { *(.dynsym) } :data
  .dynstr : { *(.dynstr) } :data
  .rel.dyn : { *(.rel.dyn) } :data
  /DISCARD/ : {
      *(.eh_frame*)
      *(.gnu.hash*)
      *(.hash*)
    }
}
";
    let mut lld_args = Vec::new();
    lld_args.push("--threads=1".to_string());
    lld_args.push("-z".to_string());
    lld_args.push("notext".to_string());
    let mut pre_link_args = BTreeMap::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), lld_args);

    TargetOptions {
        allow_asm: true,
        c_int_width: "64".to_string(),
        dll_prefix: "".to_string(),
        dynamic_linking: true,
        eh_frame_header: false,
        emit_debug_gdb_scripts: false,
        endian: Endian::Little,
        env: String::new(),
        executables: true,
        features: "+solana".to_string(),
        link_script: Some(linker_script.to_string()),
        linker: Some("rust-lld".to_owned()),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker_is_gnu: true,
        main_needs_argc_argv: false,
        max_atomic_width: Some(64),
        no_default_libraries: true,
        only_cdylib: true,
        os: "solana".to_string(),
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        pre_link_args,
        requires_lto: false,
        singlethread: true,
        vendor: "solana".to_string(),
        .. Default::default()
    }
}
